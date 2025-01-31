extern crate grammartec;
extern crate ron;
extern crate serde_json;

use grammartec::chunkstore::ChunkStore;
use grammartec::context::Context;
use grammartec::mutator::Mutator;
use grammartec::tree::{Tree, TreeLike, TreeMutation};

use std::env;
use std::fs::File;
use std::io::{self, Read};

enum MutationMethods {
    Havoc,
    HavocRec,
    Splice,
}

fn main() {
    //Parse parameters
    if env::args().len() == 5 {
        let tree_depth = env::args()
            .nth(1)
            .expect("RAND_1541841394")
            .parse::<usize>()
            .expect("RAND_1541841394");
        let tree_path = env::args().nth(2).expect("get tree_path error");
        let grammar_path = env::args().nth(3).expect("get grammar_path error");
        let method = match env::args().nth(4).expect("RAND_1161906828").as_ref() {
            "havoc" => MutationMethods::Havoc,
            "rec" => MutationMethods::HavocRec,
            "splice" => MutationMethods::Splice,
            _ => {
                panic!("Please use havoc, rec, or splice");
            }
        };
        let mut ctx = Context::new();

        //Generate rules using an antlr grammar:
        if grammar_path.ends_with(".json") {
            let gf = File::open(grammar_path).expect("cannot read grammar file");
            let rules: Vec<(String, String)> =
                serde_json::from_reader(&gf).expect("cannot parse grammar file");
            let root = "{".to_string() + &rules[0].0 + "}";
            ctx.add_rule("START", root.as_bytes());
            for rule in rules {
                ctx.add_rule(&rule.0, rule.1.as_bytes());
            }
        } else {
            panic!("Unknown grammar type");
        }

        //Deserialize tree
        let mut sf = File::open(&tree_path).expect("cannot read tree file");
        let mut tree_as_string = String::new();
        sf.read_to_string(&mut tree_as_string)
            .expect("RAND_421233044");
        let tree: Tree = ron::de::from_str(&tree_as_string).expect("Failed to deserialize tree");

        //Initialize Context
        ctx.initialize(tree_depth);

        println!(
            "Original tree:\nRules: {:?}\nSizes: {:?}\nParents: {:?}\nUnparsed original tree: ",
            tree.rules, tree.sizes, tree.paren
        );
        {
            let stdout = io::stdout();
            let mut stdout_handle = stdout.lock();
            tree.unparse_to(&ctx, &mut stdout_handle);
        }
        println!();
        let mut mutator = Mutator::new(&ctx);
        let mut tester = |tree_mut: &TreeMutation, ctx: &Context| -> Result<(), ()> {
            println!("prefix: {:?}", tree_mut.prefix);
            println!("repl: {:?}", tree_mut.repl);
            println!("postfix: {:?}", tree_mut.postfix);
            let mutated_tree = tree_mut.to_tree(ctx);
            println!(
                "Mutated tree:\nRules: {:?}\nSizes: {:?}\nParents: {:?}\nUnparsed original tree: ",
                mutated_tree.rules, mutated_tree.sizes, mutated_tree.paren
            );
            let stdout = io::stdout();
            let mut stdout_handle = stdout.lock();
            mutated_tree.unparse_to(ctx, &mut stdout_handle);
            Ok(())
        };
        match method {
            MutationMethods::Havoc => mutator
                .mut_random(&tree, &ctx, &mut tester)
                .expect("RAND_1926416364"),
            MutationMethods::HavocRec => {
                if let Some(ref mut recursions) = tree.calc_recursions(&ctx) {
                    mutator
                        .mut_random_recursion(&tree, recursions, &ctx, &mut tester)
                        .expect("RAND_1905760160");
                }
            }
            MutationMethods::Splice => {
                let mut cks = ChunkStore::new("/tmp/".to_string());
                cks.add_tree(tree.clone(), &ctx);
                mutator
                    .mut_splice(&tree, &ctx, &cks, &mut tester)
                    .expect("RAND_842617595");
            }
        }
        println!();
    } else {
        println!("Usage: generator tree_size path_to_serialized_tree path_to_grammar mutation_method(havoc, rec, splice)");
    }
}

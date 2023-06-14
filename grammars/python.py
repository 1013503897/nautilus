#从起始符号START开始，可以通过一个名为PROGRAM的非终结符号扩展出更多的代码
ctx.rule(u'START',u'{PROGRAM}')

# 这表示一个PROGRAM(完整的程序)可以由一个STATEMENT和一个换行符组成，后面紧跟着另一个PROGRAM。
ctx.rule(u'PROGRAM',u'{STATEMENT}\n{PROGRAM}')

#PROGRAM 整个程序
ctx.rule(u'PROGRAM',u'')

# 单行语句
# ctx.rule(u'STATEMENT',u'pass')
# ctx.rule(u'STATEMENT',u'break')


#变量被赋值为表达式
ctx.rule(u'STATEMENT',u'{VAR} = {EXPR}')
# 表达式也可以是变量
ctx.rule(u'EXPR',u'{VAR}')
# --------------------------------------变量-----------------------------------
ctx.rule(u'VAR',u'a')
ctx.rule(u'VAR',u'b')
ctx.rule(u'VAR',u'c')
ctx.rule(u'VAR',u'd')
ctx.rule(u'VAR',u'e')
ctx.rule(u'VAR',u'h')
ctx.rule(u'VAR',u'iNum')
ctx.rule(u'VAR',u'iIndex')
ctx.rule(u'VAR',u'strName')
ctx.rule(u'VAR',u'fValue')
ctx.rule(u'VAR',u'lstData')
ctx.rule(u'VAR',u'dicInfo')
ctx.rule(u'VAR',u'boolFlag')
ctx.rule(u'VAR',u'iList')
ctx.rule(u'VAR',u'iArr')
ctx.rule(u'VAR',u'sArr')
ctx.rule(u'VAR',u'sList')
ctx.rule(u'VAR',u'iDic')
ctx.rule(u'VAR',u'sDic')
ctx.rule(u'VAR',u'iMap')
ctx.rule(u'VAR',u'sMap')

# ---------------------------------------bool值----------------------------

ctx.rule(u'EXPR',u'{BOOLEN}')
ctx.rule(u'BOOLEN',u'None')
ctx.rule(u'BOOLEN',u'False') 
ctx.rule(u'BOOLEN',u'True')



# ---------------------------------------整型--------------------------------------
ctx.rule(u'EXPR',u'({NUMERAL})')
# 十进制数
ctx.rule(u'NUMERAL',u'{DECIMAL}')


# 整形的组合定义
ctx.rule(u'DECIMAL',u'{DECIMALDIGIT}{DECIMALDIGITS}.{DECIMALDIGIT}{DECIMALDIGITS}e-{DECIMALDIGIT}{DECIMALDIGITS}')

# 两个数字拼一起
ctx.rule(u'DECIMAL',u'{DECIMALDIGIT}{DECIMALDIGITS}')
# 这里拼出来是十六进制
ctx.rule(u'DECIMAL',u'{DECIMALDIGIT}{DECIMALDIGITS}e{DECIMALDIGIT}{DECIMALDIGITS}')
# 这里拼出来是小数
ctx.rule(u'DECIMAL',u'{DECIMALDIGIT}{DECIMALDIGITS}e-{DECIMALDIGIT}{DECIMALDIGITS}')
ctx.rule(u'DECIMAL',u'{DECIMALDIGIT}{DECIMALDIGITS}.{DECIMALDIGIT}{DECIMALDIGITS}')
ctx.rule(u'DECIMAL',u'{DECIMALDIGIT}{DECIMALDIGITS}.{DECIMALDIGIT}{DECIMALDIGITS}e{DECIMALDIGIT}{DECIMALDIGITS}')

# 定义了整形
ctx.rule(u'DECIMALDIGIT',u'0')
ctx.rule(u'DECIMALDIGIT',u'1')
ctx.rule(u'DECIMALDIGIT',u'2')
ctx.rule(u'DECIMALDIGIT',u'3')
ctx.rule(u'DECIMALDIGIT',u'4')
ctx.rule(u'DECIMALDIGIT',u'5')
ctx.rule(u'DECIMALDIGIT',u'6')
ctx.rule(u'DECIMALDIGIT',u'7')
ctx.rule(u'DECIMALDIGIT',u'8')
ctx.rule(u'DECIMALDIGIT',u'9')

# 两个单数
ctx.rule(u'DECIMALDIGITS',u'{DECIMALDIGIT}{DECIMALDIGITS}')
ctx.rule(u'DECIMALDIGITS',u'')

# 十六进制数
ctx.rule(u'NUMERAL',u'0x{HEXADECIMAL}')
ctx.rule(u'HEXADECIMAL',u'{HEXDIGIT}{HEXDIGITS}')


# 十六进制数的具体定义(多个十六进制)
ctx.rule(u'HEXDIGITS',u'{HEXDIGIT}{HEXDIGITS}')
ctx.rule(u'HEXDIGITS',u'')
ctx.rule(u'HEXDIGIT',u'a')
ctx.rule(u'HEXDIGIT',u'b')
ctx.rule(u'HEXDIGIT',u'c')
ctx.rule(u'HEXDIGIT',u'd')
ctx.rule(u'HEXDIGIT',u'e')
ctx.rule(u'HEXDIGIT',u'f')
ctx.rule(u'HEXDIGIT',u'A')
ctx.rule(u'HEXDIGIT',u'B')
ctx.rule(u'HEXDIGIT',u'C')
ctx.rule(u'HEXDIGIT',u'D')
ctx.rule(u'HEXDIGIT',u'E')
ctx.rule(u'HEXDIGIT',u'F')
# 十六进制可以是整形
ctx.rule(u'HEXDIGIT',u'{DECIMALDIGIT}')

# ---------------------------------------字符串--------------------------------------
ctx.rule(u'EXPR',u'{LITERALSTRING}')
ctx.rule(u'LITERALSTRING',u'"{STRING}"')
ctx.rule(u'STRING',u'')
ctx.rule(u'STRING',u'{STRCHR}{STRING}')
ctx.rule(u'STRCHR',u'\n')
ctx.rule(u'STRCHR',u'\r')
ctx.rule(u'STRCHR',u' ')
ctx.rule(u'STRCHR',u'\t')
ctx.rule(u'STRCHR',u'0')
ctx.rule(u'STRCHR',u'a')
ctx.rule(u'STRCHR',u'/')
ctx.rule(u'STRCHR',u'.')
ctx.rule(u'STRCHR',u'$')
ctx.rule(u'STRCHR',u'{ESCAPESEQUENCE}')
ctx.rule(u'ESCAPESEQUENCE',u'\\a')
ctx.rule(u'ESCAPESEQUENCE',u'\\b')
ctx.rule(u'ESCAPESEQUENCE',u'\\f')
ctx.rule(u'ESCAPESEQUENCE',u'\\n')
ctx.rule(u'ESCAPESEQUENCE',u'\\r')
ctx.rule(u'ESCAPESEQUENCE',u'\\t')
ctx.rule(u'ESCAPESEQUENCE',u'\\v')
ctx.rule(u'ESCAPESEQUENCE',u'\\z')
ctx.rule(u'ESCAPESEQUENCE',u'\n')
ctx.rule(u'ESCAPESEQUENCE',u'\\x{HEXADECIMAL}')
ctx.rule(u'ESCAPESEQUENCE',u'\\u\\{{HEXADECIMAL}\\}')

# ---------------------------------------数组--------------------------------------

# 数组定义
ctx.rule(u'EXPR',u'{LIST}')

# 左中括号开始 右中括号结束
ctx.rule(u'LIST',u'[{VALUELIST}]')


# 值列表可以由多个值组成，中间用逗号隔开。
ctx.rule(u'VALUELIST',u'{VALUE},{VALUELIST}')

# 可以只有一个值
ctx.rule(u'VALUELIST',u'{VALUE}')

# 值可以是数字
ctx.rule(u'VALUE', u'{NUMERAL}')
# 值可以是字符串
ctx.rule(u'VALUE', u'{LITERALSTRING}')
# 值可以是布尔值
ctx.rule(u'VALUE', u'{BOOLEN}')


# ---------------------------------------元组--------------------------------------

# 元组定义
ctx.rule(u'EXPR',u'{TUPLE}')

# 左小括号开始 右小括号结束
ctx.rule(u'TUPLE',u'({VALUELIST})')


# 值列表可以由多个值组成，中间用逗号隔开。
ctx.rule(u'VALUELIST',u'{VALUE},{VALUELIST}')

# 可以只有一个值
ctx.rule(u'VALUELIST',u'{VALUE}')

# 值可以是数字
ctx.rule(u'VALUE', u'{NUMERAL}')
# 值可以是字符串
ctx.rule(u'VALUE', u'{LITERALSTRING}')
# 值可以是布尔值
ctx.rule(u'VALUE', u'{BOOLEN}')


# ---------------------------------------字典--------------------------------------
# 字典定义
ctx.rule(u'EXPR',u'{DICT}')

# 左花括号开始 右花括号结束
ctx.rule(u'DICT',u'\\{{FIELDLIST}\\}')

# 字段列表可以由多个字段组成，中间用逗号隔开。
ctx.rule(u'FIELDLIST',u'{FIELD},{FIELDLIST}')

# 可以只有一个字段
ctx.rule(u'FIELDLIST',u'{FIELD}')


# 字段可以是变量:数字
ctx.rule("FIELD", u"\'{VAR}\' : {NUMERAL}")
# 字段可以是变量:字符串
ctx.rule("FIELD", u"\'{VAR}\' : {LITERALSTRING}")
# 字段可以是变量:布尔值
ctx.rule("FIELD", u"\'{VAR}\' : {BOOLEN}")

# ---------------------------------------函数--------------------------------------

# 函数定义
ctx.rule(u'STATEMENT', u'{FUNCTION}')

# 函数包括 函数定义 参数列表 函数体
ctx.rule(u'FUNCTION', u'def {IDENTIFIER}({FUNCTION_ARGS}):\n{INDENT}{PROGRAM}')

# 函数参数可以为空
ctx.rule(u'FUNCTION_ARGS', u'')
# 函数参数
ctx.rule(u'FUNCTION_ARGS', u'{FUNCTION_ARGLIST}')
# 函数参数列表
ctx.rule(u'FUNCTION_ARGLIST', u'{VAR}, {FUNCTION_ARGLIST}')
ctx.rule(u'FUNCTION_ARGLIST', u'{VAR}')
ctx.rule(u'FUNCTION_ARGLIST', u'*{VAR}')
ctx.rule(u'FUNCTION_ARGLIST', u'**{VAR}')

# 条件语句
ctx.rule(u'STATEMENT', u'{CONDITIONAL}')
ctx.rule(u'CONDITIONAL', u'if {EXPR}: \n{INDENT}{PROGRAM}\n')
ctx.rule(u'CONDITIONAL', u'if {EXPR}: \n{INDENT}{PROGRAM}\nelse: \n{INDENT}{PROGRAM}\n')

# 定义缩进级别
ctx.rule(u'INDENT', u'    ')
ctx.rule(u'DEDENT', u'')

# 定义循环
ctx.rule(u'STATEMENT', u'{LOOP}')
ctx.rule(u'LOOP', u'while {EXPR}:\n{INDENT}{PROGRAM}\n{DEDENT}')
ctx.rule(u'LOOP', u'for {VAR} in {EXPR}:\n{INDENT}{PROGRAM}\n{DEDENT}')
ctx.rule(u'LOOP', u'for {VAR} in range({EXPR}, {EXPR}, {EXPR}):\n{INDENT}{PROGRAM}\n{DEDENT}')

# 定义返回值 
ctx.rule(u'STATEMENT',u'return {EXPRLIST}')
ctx.rule(u'EXPRLIST',u'{EXPR}, {EXPRLIST}')
ctx.rule(u'EXPRLIST',u'{EXPR}')

# 函数调用
ctx.rule(u'FUNCTIONCALL', u'{IDENTIFIER}({ARGSLIST})')
ctx.rule(u'FUNCTIONCALL', u'{EXPR}.{IDENTIFIER}({ARGSLIST})')
ctx.rule(u'FUNCTIONCALL', u'{EXPR}({ARGSLIST})')
ctx.rule(u'ARGSLIST', u'{ARGS}, {ARGSLIST}')
ctx.rule(u'ARGSLIST', u'{ARGS}')
ctx.rule(u'ARGS', u'{EXPR}')
ctx.rule(u'ARGS', u'{EXPR}, {ARGS}')

# 定义函数名
ctx.rule(u'IDENTIFIER',u'self')
ctx.rule(u'IDENTIFIER',u'G')
ctx.rule(u'IDENTIFIER',u'_VERSION')
ctx.rule(u'IDENTIFIER',u'assert')
ctx.rule(u'IDENTIFIER',u'collectgarbage')
ctx.rule(u'IDENTIFIER',u'dofile')
ctx.rule(u'IDENTIFIER',u'error')
ctx.rule(u'IDENTIFIER',u'getmetatable')
ctx.rule(u'IDENTIFIER',u'ipairs')
ctx.rule(u'IDENTIFIER',u'load')
ctx.rule(u'IDENTIFIER',u'loadfile')
ctx.rule(u'IDENTIFIER',u'next')
ctx.rule(u'IDENTIFIER',u'pairs')
ctx.rule(u'IDENTIFIER',u'pcall')
ctx.rule(u'IDENTIFIER',u'print')
ctx.rule(u'IDENTIFIER',u'rawequal')
ctx.rule(u'IDENTIFIER',u'rawget')
ctx.rule(u'IDENTIFIER',u'rawlen')
ctx.rule(u'IDENTIFIER',u'rawset')
ctx.rule(u'IDENTIFIER',u'require')
ctx.rule(u'IDENTIFIER',u'select')
ctx.rule(u'IDENTIFIER',u'setmetatable')
ctx.rule(u'IDENTIFIER',u'tonumber')
ctx.rule(u'IDENTIFIER',u'tostring')
ctx.rule(u'IDENTIFIER',u'type')
ctx.rule(u'IDENTIFIER',u'xpcall')
ctx.rule(u'IDENTIFIER',u'coroutine')
ctx.rule(u'IDENTIFIER',u'create')
ctx.rule(u'IDENTIFIER',u'isyieldable')
ctx.rule(u'IDENTIFIER',u'resume')
ctx.rule(u'IDENTIFIER',u'running')
ctx.rule(u'IDENTIFIER',u'status')
ctx.rule(u'IDENTIFIER',u'wrap')
ctx.rule(u'IDENTIFIER',u'yield')
ctx.rule(u'IDENTIFIER',u'debug')
ctx.rule(u'IDENTIFIER',u'debug')
ctx.rule(u'IDENTIFIER',u'gethook')
ctx.rule(u'IDENTIFIER',u'getinfo')
ctx.rule(u'IDENTIFIER',u'getlocal')
ctx.rule(u'IDENTIFIER',u'getmetatable')
ctx.rule(u'IDENTIFIER',u'getregistry')
ctx.rule(u'IDENTIFIER',u'getupvalue')
ctx.rule(u'IDENTIFIER',u'getuservalue')
ctx.rule(u'IDENTIFIER',u'sethook')
ctx.rule(u'IDENTIFIER',u'setlocal')
ctx.rule(u'IDENTIFIER',u'setmetatable')
ctx.rule(u'IDENTIFIER',u'setupvalue')
ctx.rule(u'IDENTIFIER',u'setuservalue')
ctx.rule(u'IDENTIFIER',u'traceback')
ctx.rule(u'IDENTIFIER',u'upvalueid')
ctx.rule(u'IDENTIFIER',u'upvaluejoin')
ctx.rule(u'IDENTIFIER',u'io')
ctx.rule(u'IDENTIFIER',u'close')
ctx.rule(u'IDENTIFIER',u'flush')
ctx.rule(u'IDENTIFIER',u'input')
ctx.rule(u'IDENTIFIER',u'lines')
ctx.rule(u'IDENTIFIER',u'open')
ctx.rule(u'IDENTIFIER',u'output')
ctx.rule(u'IDENTIFIER',u'popen')
ctx.rule(u'IDENTIFIER',u'read')
ctx.rule(u'IDENTIFIER',u'stderr')
ctx.rule(u'IDENTIFIER',u'stdin')
ctx.rule(u'IDENTIFIER',u'stdout')
ctx.rule(u'IDENTIFIER',u'tmpfile')
ctx.rule(u'IDENTIFIER',u'type')
ctx.rule(u'IDENTIFIER',u'write')
ctx.rule(u'IDENTIFIER',u'math')
ctx.rule(u'IDENTIFIER',u'abs')
ctx.rule(u'IDENTIFIER',u'acos')
ctx.rule(u'IDENTIFIER',u'asin')
ctx.rule(u'IDENTIFIER',u'atan')
ctx.rule(u'IDENTIFIER',u'ceil')
ctx.rule(u'IDENTIFIER',u'cos')
ctx.rule(u'IDENTIFIER',u'deg')
ctx.rule(u'IDENTIFIER',u'exp')
ctx.rule(u'IDENTIFIER',u'floor')
ctx.rule(u'IDENTIFIER',u'fmod')
ctx.rule(u'IDENTIFIER',u'huge')
ctx.rule(u'IDENTIFIER',u'log')
ctx.rule(u'IDENTIFIER',u'max')
ctx.rule(u'IDENTIFIER',u'maxinteger')
ctx.rule(u'IDENTIFIER',u'min')
ctx.rule(u'IDENTIFIER',u'mininteger')
ctx.rule(u'IDENTIFIER',u'modf')
ctx.rule(u'IDENTIFIER',u'pi')
ctx.rule(u'IDENTIFIER',u'rad')
ctx.rule(u'IDENTIFIER',u'random')
ctx.rule(u'IDENTIFIER',u'randomseed')
ctx.rule(u'IDENTIFIER',u'sin')
ctx.rule(u'IDENTIFIER',u'sqrt')
ctx.rule(u'IDENTIFIER',u'tan')
ctx.rule(u'IDENTIFIER',u'tointeger')
ctx.rule(u'IDENTIFIER',u'type')
ctx.rule(u'IDENTIFIER',u'ult')
ctx.rule(u'IDENTIFIER',u'os')
ctx.rule(u'IDENTIFIER',u'clock')
ctx.rule(u'IDENTIFIER',u'date')
ctx.rule(u'IDENTIFIER',u'difftime')
ctx.rule(u'IDENTIFIER',u'exit')
ctx.rule(u'IDENTIFIER',u'getenv')
ctx.rule(u'IDENTIFIER',u'remove')
ctx.rule(u'IDENTIFIER',u'rename')
ctx.rule(u'IDENTIFIER',u'setlocale')
ctx.rule(u'IDENTIFIER',u'time')
ctx.rule(u'IDENTIFIER',u'tmpname')
ctx.rule(u'IDENTIFIER',u'package')
ctx.rule(u'IDENTIFIER',u'config')
ctx.rule(u'IDENTIFIER',u'cpath')
ctx.rule(u'IDENTIFIER',u'loaded')
ctx.rule(u'IDENTIFIER',u'loadlib')
ctx.rule(u'IDENTIFIER',u'path')
ctx.rule(u'IDENTIFIER',u'preload')
ctx.rule(u'IDENTIFIER',u'searchers')
ctx.rule(u'IDENTIFIER',u'searchpath')
ctx.rule(u'IDENTIFIER',u'string')
ctx.rule(u'IDENTIFIER',u'byte')
ctx.rule(u'IDENTIFIER',u'char')
ctx.rule(u'IDENTIFIER',u'dump')
ctx.rule(u'IDENTIFIER',u'find')
ctx.rule(u'IDENTIFIER',u'format')
ctx.rule(u'IDENTIFIER',u'gmatch') 
ctx.rule(u'IDENTIFIER',u'gsub')
ctx.rule(u'IDENTIFIER',u'len')
ctx.rule(u'IDENTIFIER',u'lower')
ctx.rule(u'IDENTIFIER',u'match')
ctx.rule(u'IDENTIFIER',u'pack')
ctx.rule(u'IDENTIFIER',u'packsize')
ctx.rule(u'IDENTIFIER',u'rep')
ctx.rule(u'IDENTIFIER',u'reverse')
ctx.rule(u'IDENTIFIER',u'sub')
ctx.rule(u'IDENTIFIER',u'unpack')
ctx.rule(u'IDENTIFIER',u'upper')
ctx.rule(u'IDENTIFIER',u'table')
ctx.rule(u'IDENTIFIER',u'concat')
ctx.rule(u'IDENTIFIER',u'insert')
ctx.rule(u'IDENTIFIER',u'move')
ctx.rule(u'IDENTIFIER',u'pack')
ctx.rule(u'IDENTIFIER',u'remove')
ctx.rule(u'IDENTIFIER',u'sort')
ctx.rule(u'IDENTIFIER',u'unpack')
ctx.rule(u'IDENTIFIER',u'utf8')
ctx.rule(u'IDENTIFIER',u'char')
ctx.rule(u'IDENTIFIER',u'charpattern')
ctx.rule(u'IDENTIFIER',u'codepoint')
ctx.rule(u'IDENTIFIER',u'codes')
ctx.rule(u'IDENTIFIER',u'len')
ctx.rule(u'IDENTIFIER',u'offset')
ctx.rule(u'IDENTIFIER',u'create')
ctx.rule(u'IDENTIFIER',u'isyieldable')
ctx.rule(u'IDENTIFIER',u'resume')
ctx.rule(u'IDENTIFIER',u'running')
ctx.rule(u'IDENTIFIER',u'status')
ctx.rule(u'IDENTIFIER',u'wrap')
ctx.rule(u'IDENTIFIER',u'yield')
ctx.rule(u'IDENTIFIER',u'debug')
ctx.rule(u'IDENTIFIER',u'gethook')
ctx.rule(u'IDENTIFIER',u'getinfo')
ctx.rule(u'IDENTIFIER',u'getlocal')
ctx.rule(u'IDENTIFIER',u'getmetatable')
ctx.rule(u'IDENTIFIER',u'getregistry')
ctx.rule(u'IDENTIFIER',u'getupvalue')
ctx.rule(u'IDENTIFIER',u'getuservalue')
ctx.rule(u'IDENTIFIER',u'sethook')
ctx.rule(u'IDENTIFIER',u'setlocal')
ctx.rule(u'IDENTIFIER',u'setmetatable')
ctx.rule(u'IDENTIFIER',u'setupvalue')
ctx.rule(u'IDENTIFIER',u'setuservalue')
ctx.rule(u'IDENTIFIER',u'traceback')
ctx.rule(u'IDENTIFIER',u'upvalueid')
ctx.rule(u'IDENTIFIER',u'upvaluejoin')
ctx.rule(u'IDENTIFIER',u'close')
ctx.rule(u'IDENTIFIER',u'flush')
ctx.rule(u'IDENTIFIER',u'input')
ctx.rule(u'IDENTIFIER',u'lines')
ctx.rule(u'IDENTIFIER',u'open')
ctx.rule(u'IDENTIFIER',u'output')
ctx.rule(u'IDENTIFIER',u'popen')
ctx.rule(u'IDENTIFIER',u'read')
ctx.rule(u'IDENTIFIER',u'stderr')
ctx.rule(u'IDENTIFIER',u'stdin')
ctx.rule(u'IDENTIFIER',u'stdout')
ctx.rule(u'IDENTIFIER',u'tmpfile')
ctx.rule(u'IDENTIFIER',u'type')
ctx.rule(u'IDENTIFIER',u'write')
ctx.rule(u'IDENTIFIER',u'close')
ctx.rule(u'IDENTIFIER',u'flush')
ctx.rule(u'IDENTIFIER',u'lines')
ctx.rule(u'IDENTIFIER',u'read')
ctx.rule(u'IDENTIFIER',u'seek')
ctx.rule(u'IDENTIFIER',u'setvbuf')
ctx.rule(u'IDENTIFIER',u'write')
ctx.rule(u'IDENTIFIER',u'abs')
ctx.rule(u'IDENTIFIER',u'acos')
ctx.rule(u'IDENTIFIER',u'asin')
ctx.rule(u'IDENTIFIER',u'atan')
ctx.rule(u'IDENTIFIER',u'ceil')
ctx.rule(u'IDENTIFIER',u'cos')
ctx.rule(u'IDENTIFIER',u'deg')
ctx.rule(u'IDENTIFIER',u'exp')
ctx.rule(u'IDENTIFIER',u'floor')
ctx.rule(u'IDENTIFIER',u'fmod')
ctx.rule(u'IDENTIFIER',u'huge')
ctx.rule(u'IDENTIFIER',u'log')
ctx.rule(u'IDENTIFIER',u'max')
ctx.rule(u'IDENTIFIER',u'maxinteger')
ctx.rule(u'IDENTIFIER',u'min')
ctx.rule(u'IDENTIFIER',u'mininteger')
ctx.rule(u'IDENTIFIER',u'modf')
ctx.rule(u'IDENTIFIER',u'pi')
ctx.rule(u'IDENTIFIER',u'rad')
ctx.rule(u'IDENTIFIER',u'random')
ctx.rule(u'IDENTIFIER',u'randomseed')
ctx.rule(u'IDENTIFIER',u'sin')
ctx.rule(u'IDENTIFIER',u'sqrt')
ctx.rule(u'IDENTIFIER',u'tan')
ctx.rule(u'IDENTIFIER',u'tointeger')
ctx.rule(u'IDENTIFIER',u'type')
ctx.rule(u'IDENTIFIER',u'ult')
ctx.rule(u'IDENTIFIER',u'clock')
ctx.rule(u'IDENTIFIER',u'date')
ctx.rule(u'IDENTIFIER',u'difftime')
ctx.rule(u'IDENTIFIER',u'exit')
ctx.rule(u'IDENTIFIER',u'getenv')
ctx.rule(u'IDENTIFIER',u'remove')
ctx.rule(u'IDENTIFIER',u'rename')
ctx.rule(u'IDENTIFIER',u'setlocale')
ctx.rule(u'IDENTIFIER',u'time')
ctx.rule(u'IDENTIFIER',u'tmpname')
ctx.rule(u'IDENTIFIER',u'config')
ctx.rule(u'IDENTIFIER',u'cpath')
ctx.rule(u'IDENTIFIER',u'loaded')
ctx.rule(u'IDENTIFIER',u'loadlib')
ctx.rule(u'IDENTIFIER',u'path')
ctx.rule(u'IDENTIFIER',u'preload')
ctx.rule(u'IDENTIFIER',u'searchers')
ctx.rule(u'IDENTIFIER',u'searchpath')
ctx.rule(u'IDENTIFIER',u'byte')
ctx.rule(u'IDENTIFIER',u'char')
ctx.rule(u'IDENTIFIER',u'dump')
ctx.rule(u'IDENTIFIER',u'find')
ctx.rule(u'IDENTIFIER',u'format')
ctx.rule(u'IDENTIFIER',u'gmatch')
ctx.rule(u'IDENTIFIER',u'gsub')
ctx.rule(u'IDENTIFIER',u'len')
ctx.rule(u'IDENTIFIER',u'lower')
ctx.rule(u'IDENTIFIER',u'match')
ctx.rule(u'IDENTIFIER',u'pack')
ctx.rule(u'IDENTIFIER',u'packsize')
ctx.rule(u'IDENTIFIER',u'rep')
ctx.rule(u'IDENTIFIER',u'reverse')
ctx.rule(u'IDENTIFIER',u'sub')
ctx.rule(u'IDENTIFIER',u'unpack')
ctx.rule(u'IDENTIFIER',u'upper')
ctx.rule(u'IDENTIFIER',u'concat')
ctx.rule(u'IDENTIFIER',u'insert')
ctx.rule(u'IDENTIFIER',u'move')
ctx.rule(u'IDENTIFIER',u'pack')
ctx.rule(u'IDENTIFIER',u'remove')
ctx.rule(u'IDENTIFIER',u'sort')
ctx.rule(u'IDENTIFIER',u'unpack')
ctx.rule(u'IDENTIFIER',u'char')
ctx.rule(u'IDENTIFIER',u'charpattern')
ctx.rule(u'IDENTIFIER',u'codepoint')
ctx.rule(u'IDENTIFIER',u'codes')
ctx.rule(u'IDENTIFIER',u'len')
ctx.rule(u'IDENTIFIER',u'offset')
ctx.rule(u'IDENTIFIER',u'__index')
ctx.rule(u'IDENTIFIER',u'__newindex')
ctx.rule(u'IDENTIFIER',u'__add')
ctx.rule(u'IDENTIFIER',u'__sub')
ctx.rule(u'IDENTIFIER',u'__mul')
ctx.rule(u'IDENTIFIER',u'__div')
ctx.rule(u'IDENTIFIER',u'__mod')
ctx.rule(u'IDENTIFIER',u'__unm')
ctx.rule(u'IDENTIFIER',u'__concat')
ctx.rule(u'IDENTIFIER',u'__eq')
ctx.rule(u'IDENTIFIER',u'__lt')
ctx.rule(u'IDENTIFIER',u'__le')
ctx.rule(u'IDENTIFIER',u'__call')
ctx.rule(u'IDENTIFIER',u'__tostring')
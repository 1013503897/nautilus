use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    /**
     * 容器id
     */
    #[serde(rename = "containerId")]
    pub container_id: String,

    /**
     * 代码行覆盖率
     */
    #[serde(rename = "lineCoverage")]
    pub line_coverage: String,

    /**
     * 函数覆盖率
     */
    #[serde(rename = "functionCoverage")]
    pub function_coverage: String,

    /**
     * 最后一次崩溃时间
     */
    #[serde(rename = "lastCrashTime")]
    pub last_crash_time: String,

    /**
     * 最后一次超时时间
     */
    #[serde(rename = "lastTimeoutTime")]
    pub last_timeout_time: String,

    /**
     * 哈希表密度
     */
    #[serde(rename = "mapDensity")]
    pub map_density: f32,

    /**
     * 运行样本数
     */
    #[serde(rename = "sampleCount")]
    pub sample_count: u64,

    /**
     * 崩溃样本数
     */
    #[serde(rename = "crashCount")]
    pub crash_count: u64,

    /**
     * 样本运行速率
     */
    #[serde(rename = "sampleRunRate")]
    pub sample_run_rate: f32,
}

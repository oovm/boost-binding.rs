const ITERATIONS = 10000;

interface BenchmarkResult {
    name: string;
    mean: number;
    min: number;
    max: number;
    median: number;
    stdDev: number;
    iterations: number;
}

interface CLIParser {
    name: string;
    parse(args: string[]): any;
}

class Benchmarker {
    private results: BenchmarkResult[] = [];

    async measure(name: string, fn: () => void): Promise<void> {
        const times: number[] = [];

        console.log(`\n=== 正在测试: ${name} ===`);
        console.log(`执行 ${ITERATIONS} 次迭代...`);

        for (let i = 0; i < ITERATIONS; i++) {
            const start = performance.now();
            fn();
            const end = performance.now();
            const duration = end - start;
            times.push(duration);
            
            if (i % 1000 === 0) {
                console.log(`  迭代 ${i + 1}: ${duration.toFixed(2)}ms`);
            }
        }

        const result = this.calculateStats(name, times);
        this.results.push(result);
        this.printResult(result);
    }

    private calculateStats(name: string, times: number[]): BenchmarkResult {
        const sorted = [...times].sort((a, b) => a - b);
        const sum = times.reduce((a, b) => a + b, 0);
        const mean = sum / times.length;
        const min = sorted[0];
        const max = sorted[sorted.length - 1];
        const median = 
            sorted.length % 2 === 0
                ? (sorted[sorted.length / 2 - 1] + sorted[sorted.length / 2]) / 2
                : sorted[Math.floor(sorted.length / 2)];
        const variance = times.reduce((acc, t) => acc + Math.pow(t - mean, 2), 0) / times.length;
        const stdDev = Math.sqrt(variance);

        return {
            name,
            mean,
            min,
            max,
            median,
            stdDev,
            iterations: times.length,
        };
    }

    private printResult(result: BenchmarkResult): void {
        console.log(`\n--- ${result.name} 结果 ---`);
        console.log(`  平均: ${result.mean.toFixed(2)}ms`);
        console.log(`  中位数: ${result.median.toFixed(2)}ms`);
        console.log(`  最小: ${result.min.toFixed(2)}ms`);
        console.log(`  最大: ${result.max.toFixed(2)}ms`);
        console.log(`  标准差: ${result.stdDev.toFixed(2)}ms`);
    }

    printReport(): void {
        console.log("\n" + "=".repeat(60));
        console.log("             CLI 解析库性能基准测试报告");
        console.log("=".repeat(60));

        console.log(
            "\n".padEnd(22, " ") +
                "平均(ms)".padEnd(12) +
                "中位数(ms)".padEnd(14) +
                "最小(ms)".padEnd(12) +
                "最大(ms)".padEnd(12) +
                "标准差(ms)",
        );
        console.log("-".repeat(85));

        for (const result of this.results) {
            console.log(
                result.name.padEnd(22) +
                    result.mean.toFixed(2).padStart(10) +
                    "ms".padEnd(2) +
                    result.median.toFixed(2).padStart(12) +
                    "ms".padEnd(2) +
                    result.min.toFixed(2).padStart(10) +
                    "ms".padEnd(2) +
                    result.max.toFixed(2).padStart(10) +
                    "ms".padEnd(2) +
                    result.stdDev.toFixed(2).padStart(10) +
                    "ms",
            );
        }

        console.log("\n" + "=".repeat(60));
    }
}

// 模拟 CLI 解析库的实现
class MockCLIParser implements CLIParser {
    name: string;

    constructor(name: string) {
        this.name = name;
    }

    parse(args: string[]): any {
        // 简单的模拟解析逻辑
        const result: any = {};
        let currentKey: string | null = null;

        for (const arg of args) {
            if (arg.startsWith("--")) {
                currentKey = arg.slice(2);
                result[currentKey] = true;
            } else if (currentKey) {
                result[currentKey] = arg;
                currentKey = null;
            } else {
                result["command"] = arg;
            }
        }

        return result;
    }
}

// 测试场景
type TestScenario = {
    name: string;
    args: string[];
};

function createTestScenarios(): TestScenario[] {
    return [
        {
            name: "简单命令",
            args: ["init", "project-name"]
        },
        {
            name: "带选项的命令",
            args: ["build", "--target", "production", "--watch"]
        },
        {
            name: "嵌套命令",
            args: ["config", "set", "user.name", "John Doe"]
        },
        {
            name: "大量参数",
            args: [
                "run", "build",
                "--config", "webpack.config.js",
                "--mode", "production",
                "--output-path", "./dist",
                "--public-path", "/",
                "--source-map",
                "--minify",
                "--analyze"
            ]
        }
    ];
}

async function main(): Promise<void> {
    console.log("=".repeat(60));
    console.log("  CLI 解析库性能基准测试");
    console.log("=".repeat(60));
    console.log(`测试迭代次数: ${ITERATIONS}`);

    const benchmarker = new Benchmarker();
    const scenarios = createTestScenarios();
    const parsers: CLIParser[] = [
        new MockCLIParser("Mock Parser")
    ];

    try {
        for (const parser of parsers) {
            for (const scenario of scenarios) {
                await benchmarker.measure(`${parser.name} - ${scenario.name}`, () => {
                    parser.parse(scenario.args);
                });
            }
        }

        benchmarker.printReport();

        console.log("\n注意: 当前版本只测试了 Mock Parser。");
        console.log("后续版本将添加更多 CLI 解析库进行测试。");
    } catch (error) {
        console.error("测试过程中发生错误:", error);
        process.exit(1);
    }
}

main().catch(console.error);

extern crate rustatsd;

use rustatsd::MetricIngester;

fn main() {
    let metric_ingester = MetricIngester::new();
    metric_ingester.run();
}

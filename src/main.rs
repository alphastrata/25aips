mod lib;

use lib::CarCountsCollection;

fn main() {
    println!("Hello, assesors!\n");

    let data = CarCountsCollection::new_from_disk("data.txt");
    //• The number of cars seen in total
    print!("Total cars counted:");
    data.print_total_cars_counted();

    //• A sequence of lines where each line contains a date (in yyyy-mm-dd format) and the number of cars seen on that day (eg. 2016-11-23 289) for all days listed in the input file.
    println!("\nTotals by day:");
    data.print_total_cars_counted_by_day();

    println!("\nTop three half hours with the most cars:");
    data.print_top_three_counts();

    println!("\nThe 1.5 hour period with least cars:");
    data.print_lowest_90min_total();
}

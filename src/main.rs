use std::io;
use genpdf::{elements, Alignment, Document, Element as _};

fn calculate_average(total_marks: f32, num_subjects: f32) -> f32 {
    total_marks / num_subjects
}

fn assign_grade(avg: f32) -> &'static str {
    match avg {
        x if x >= 90.0 => "A",
        x if x >= 75.0 => "B",
        x if x >= 60.0 => "C",
        _ => "D",
    }
}

fn main() {
    let mut name = String::new();
    let mut total_marks = String::new();
    let mut num_subjects = String::new();

    println!("Enter student name:");
    io::stdin().read_line(&mut name).unwrap();

    println!("Enter total marks:");
    io::stdin().read_line(&mut total_marks).unwrap();

    println!("Enter number of subjects:");
    io::stdin().read_line(&mut num_subjects).unwrap();

    let total: f32 = total_marks.trim().parse().unwrap();
    let subjects: f32 = num_subjects.trim().parse().unwrap();

    let avg = calculate_average(total, subjects);
    let grade = assign_grade(avg);

    println!("\n--- Report Card ---");
    println!("Name: {}", name.trim());
    println!("Total Marks: {}", total);
    println!("Average: {:.2}", avg);
    println!("Grade: {}", grade);

    // Create PDF
    let mut doc = Document::new(genpdf::fonts::from_files("./fonts", "LiberationSans", None).unwrap());
    doc.set_title("Student Report Card");

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    let mut report = elements::LinearLayout::vertical();
    report.push(elements::Paragraph::new("Student Report Card").aligned(Alignment::Center));
    report.push(elements::Break::new(1));
    report.push(elements::Paragraph::new(format!("Name: {}", name.trim())));
    report.push(elements::Paragraph::new(format!("Total Marks: {}", total)));
    report.push(elements::Paragraph::new(format!("Average: {:.2}", avg)));
    report.push(elements::Paragraph::new(format!("Grade: {}", grade)));

    doc.push(report);
    doc.render_to_file("report_card.pdf").expect("Failed to write PDF");
}

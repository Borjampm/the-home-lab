use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Gauge, Paragraph, Sparkline},
    Frame,
};

use crate::app::{App, Panel};

// Catppuccin Mocha palette
const BG: Color = Color::Rgb(30, 30, 46);
const FG: Color = Color::Rgb(205, 214, 244);
const GREEN: Color = Color::Rgb(166, 227, 161);
const RED: Color = Color::Rgb(243, 139, 168);
const BLUE: Color = Color::Rgb(137, 180, 250);
const YELLOW: Color = Color::Rgb(249, 226, 175);
const LAVENDER: Color = Color::Rgb(180, 190, 254);
const SURFACE0: Color = Color::Rgb(49, 50, 68);

pub fn draw(frame: &mut Frame, app: &App) {
    let size = frame.area();

    let status = if app.connected {
        Span::styled("[Connected]", Style::default().fg(GREEN))
    } else {
        Span::styled("[Disconnected]", Style::default().fg(RED))
    };

    let title_line = Line::from(vec![
        Span::styled(
            " node-tui ",
            Style::default()
                .fg(LAVENDER)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("─── "),
        status,
        Span::raw(" "),
    ]);

    let help_line = Line::from(vec![
        Span::styled(" [c]", Style::default().fg(YELLOW)),
        Span::raw(" Reconnect  "),
        Span::styled("[Tab]", Style::default().fg(YELLOW)),
        Span::raw(" Switch panel  "),
        Span::styled("[Enter]", Style::default().fg(YELLOW)),
        Span::raw(" Send greeting  "),
        Span::styled("[q]", Style::default().fg(YELLOW)),
        Span::raw(" Quit "),
    ]);

    let outer_block = Block::bordered()
        .title_top(title_line)
        .title_bottom(help_line)
        .border_style(Style::default().fg(LAVENDER))
        .style(Style::default().bg(BG).fg(FG));

    let inner = outer_block.inner(size);
    frame.render_widget(outer_block, size);

    let [cpu_area, greeter_area] = Layout::vertical([
        Constraint::Min(6),    // CPU panel
        Constraint::Length(5), // Greeter panel
    ])
    .areas(inner);

    draw_cpu_panel(frame, app, cpu_area);
    draw_greeter_panel(frame, app, greeter_area);
}

fn draw_cpu_panel(frame: &mut Frame, app: &App, area: Rect) {
    let is_active = app.active_panel == Panel::Cpu;
    let border_color = if is_active { BLUE } else { SURFACE0 };

    let block = Block::bordered()
        .title(" CPU Monitor ")
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(BG).fg(FG));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let [stats_area, spark_area, gauge_area] = Layout::vertical([
        Constraint::Length(1), // Stats line
        Constraint::Min(1),   // Sparkline
        Constraint::Length(1), // Gauge
    ])
    .areas(inner);

    // Stats line
    let error_hint = match &app.connection_error {
        Some(e) => format!("  ({e})"),
        None => String::new(),
    };
    let stats = Paragraph::new(format!(
        " CPUs: {}  |  Usage: {:.1}%{}",
        app.cpu_count, app.cpu_usage, error_hint
    ))
    .style(Style::default().bg(BG).fg(FG));
    frame.render_widget(stats, stats_area);

    // Sparkline
    let data: Vec<u64> = app.cpu_history.iter().copied().collect();
    let sparkline = Sparkline::default()
        .data(&data)
        .max(100)
        .style(Style::default().fg(GREEN).bg(BG));
    frame.render_widget(sparkline, spark_area);

    // Gauge
    let ratio = (app.cpu_usage as f64 / 100.0).clamp(0.0, 1.0);
    let gauge = Gauge::default()
        .ratio(ratio)
        .gauge_style(Style::default().fg(GREEN).bg(SURFACE0))
        .label(format!("{:.1}%", app.cpu_usage));
    frame.render_widget(gauge, gauge_area);
}

fn draw_greeter_panel(frame: &mut Frame, app: &App, area: Rect) {
    let is_active = app.active_panel == Panel::Greeter;
    let border_color = if is_active { BLUE } else { SURFACE0 };

    let block = Block::bordered()
        .title(" Greeter ")
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(BG).fg(FG));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let [input_area, response_area] = Layout::vertical([
        Constraint::Length(1), // Input
        Constraint::Length(1), // Response
    ])
    .areas(inner);

    // Input line
    let cursor = if is_active { "_" } else { "" };
    let input = Paragraph::new(Line::from(vec![
        Span::styled(" Name: ", Style::default().fg(LAVENDER)),
        Span::raw(&app.greeter_input),
        Span::styled(
            cursor,
            Style::default()
                .fg(YELLOW)
                .add_modifier(Modifier::SLOW_BLINK),
        ),
    ]));
    frame.render_widget(input, input_area);

    // Response line
    let response_text = match &app.greeter_response {
        Some(r) => r.as_str(),
        None => "(none yet)",
    };
    let response = Paragraph::new(Line::from(vec![
        Span::styled(" Response: ", Style::default().fg(LAVENDER)),
        Span::raw(response_text),
    ]));
    frame.render_widget(response, response_area);
}

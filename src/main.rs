mod grid;
mod square_walls;
mod wall;

use flo_canvas::*;
use flo_draw::binding::BindRef;
use flo_draw::*;

use crate::grid::Grid;

use futures::executor;
use futures::prelude::*;

const BACKGROUND_FILL: Color = Color::Rgba(1.0, 1.0, 1.0, 1.0); // White
const SQUARE_FILL: [Color; 5] = [
    BACKGROUND_FILL,                 // White
    BACKGROUND_FILL,                 // White
    Color::Rgba(1.0, 1.0, 0.0, 1.0), // Yellow
    Color::Rgba(0.0, 0.0, 1.0, 1.0), // Blue
    Color::Rgba(0.0, 0.0, 0.0, 1.0), // Black
];
const DOT_FILL: Color = Color::Rgba(0.0, 0.0, 0.0, 1.0); // Black
const LINE_STROKE: Color = Color::Rgba(0.392, 0.392, 0.392, 1.0); // Gray
const OPTIMAL_MOVE_STROKE: Color = Color::Rgba(0.0, 1.0, 0.0, 1.0); // Green

const MARGIN: f32 = 0.1;

fn calculate_transform(
    width: u64,
    height: u64,
    columns: i32,
    rows: i32,
) -> (Transform2D, Transform2D) {
    let width = width as f32;
    let height = height as f32;
    let columns = columns as f32;
    let rows = rows as f32;

    let window_aspect_ratio = width / height;
    let canvas_transform = if window_aspect_ratio > (columns + MARGIN) / (rows + MARGIN) {
        // Window aspect ratio (width:height) is greater than grid aspect ratio
        // (columns:rows) therefore the grid size is bounded by height.

        // Scale to fit grid and invert y-axis (to better match original
        // implementation) then translate to center grid within window.
        println!(
            "1 Scale: {}, Translate: ({}, {})",
            2.0 / (rows + MARGIN),
            columns / -2.0 + 0.5,
            rows / -2.0 + 0.5
        );
        Transform2D::scale(2.0 / (rows + MARGIN), -2.0 / (rows + MARGIN))
            * Transform2D::translate(columns / -2.0 + 0.5, rows / -2.0 + 0.5)
    } else {
        // Window aspect ratio (width:height) is less than or equal to grid aspect ratio
        // (columns:rows) therefore the grid size is bounded by width.

        // Scale to fit grid and invert y-axis (to better match original
        // implementation) then translate to center grid within window.
        println!(
            "2 Scale: {}, Translate: ({}, {})",
            2.0 * window_aspect_ratio / (columns + MARGIN),
            columns / -2.0 + 0.5,
            rows / -2.0 + 0.5
        );
        Transform2D::scale(
            2.0 * window_aspect_ratio / (columns + MARGIN),
            -2.0 * window_aspect_ratio / (columns + MARGIN),
        ) * Transform2D::translate(columns / -2.0 + 0.5, rows / -2.0 + 0.5)
    };
    let window_transform = canvas_transform.invert().unwrap()
        * Transform2D::translate(-window_aspect_ratio, 1.0)
        * Transform2D::scale(2.0 / height, -2.0 / height);
    (canvas_transform, window_transform)
}

fn draw_square(graphics_context: &mut Vec<Draw>, column: f32, row: f32, color: Color) {
    graphics_context.layer(LayerId(0));
    graphics_context.new_path();
    graphics_context.rect(column, row, column + 1.0, row + 1.0);
    graphics_context.fill_color(color);
    graphics_context.fill();
}

fn draw_dot(graphics_context: &mut Vec<Draw>, column: f32, row: f32) {
    graphics_context.layer(LayerId(2));
    graphics_context.new_path();
    graphics_context.circle(column, row, 0.1);
    graphics_context.fill_color(DOT_FILL);
    graphics_context.fill();
}

fn draw_row(graphics_context: &mut Vec<Draw>, column: f32, row: f32, color: Color) {
    graphics_context.layer(LayerId(1));
    graphics_context.new_path();
    graphics_context.rect(column, row - 0.1, column + 1.0, row + 0.1);
    graphics_context.fill_color(color);
    graphics_context.fill();
}

fn draw_column(graphics_context: &mut Vec<Draw>, column: f32, row: f32, color: Color) {
    graphics_context.layer(LayerId(1));
    graphics_context.new_path();
    graphics_context.rect(column - 0.1, row, column + 0.1, row + 1.0);
    graphics_context.fill_color(color);
    graphics_context.fill();
}

fn redraw_all(graphics_context: &mut Vec<Draw>, transform: Transform2D, game_grid: &Grid) {
    graphics_context.clear_canvas(BACKGROUND_FILL);

    let optimal_moves = game_grid.get_optimal_moves();

    graphics_context.identity_transform();
    graphics_context.transform(transform);

    let mut column_f;
    let mut row_f;

    for column in 0..game_grid.column_count() {
        for row in 0..game_grid.row_count() {
            column_f = column as f32;
            row_f = row as f32;

            if row < game_grid.row_count() - 1 {
                if column < game_grid.column_count() - 1 {
                    // Draw a square.
                    draw_square(
                        graphics_context,
                        column_f,
                        row_f,
                        SQUARE_FILL[game_grid.get_wall_count(column, row) as usize],
                    );
                }

                // Draw a column.
                let wall = game_grid.get_wall(true, column, row);
                if optimal_moves.contains(&wall) {
                    draw_column(graphics_context, column_f, row_f, OPTIMAL_MOVE_STROKE);
                } else if wall.set {
                    draw_column(graphics_context, column_f, row_f, LINE_STROKE);
                }
            }
            if column < game_grid.column_count() - 1 {
                // Draw a row.
                let wall = game_grid.get_wall(false, column, row);
                if optimal_moves.contains(&wall) {
                    draw_row(graphics_context, column_f, row_f, OPTIMAL_MOVE_STROKE);
                } else if wall.set {
                    draw_row(graphics_context, column_f, row_f, LINE_STROKE);
                }
            }

            // Draw a dot.
            draw_dot(graphics_context, column_f, row_f);
        }
    }
}

fn redraw_lines(graphics_context: &mut Vec<Draw>, game_grid: &Grid) {
    graphics_context.layer(LayerId(1));
    graphics_context.clear_layer();

    let optimal_moves = game_grid.get_optimal_moves();
    let mut wall;
    for column in 0..game_grid.column_count() {
        for row in 0..game_grid.row_count() {
            if row < game_grid.row_count() - 1 {
                // Column lines do not exist in the last row.
                wall = game_grid.get_wall(true, column, row);
                if optimal_moves.contains(&wall) {
                    draw_column(
                        graphics_context,
                        column as f32,
                        row as f32,
                        OPTIMAL_MOVE_STROKE,
                    );
                } else if wall.set {
                    draw_column(graphics_context, column as f32, row as f32, LINE_STROKE);
                }
            }
            if column < game_grid.column_count() - 1 {
                wall = game_grid.get_wall(false, column, row);
                if optimal_moves.contains(&wall) {
                    draw_row(
                        graphics_context,
                        column as f32,
                        row as f32,
                        OPTIMAL_MOVE_STROKE,
                    );
                } else if wall.set {
                    draw_row(graphics_context, column as f32, row as f32, LINE_STROKE);
                }
            }
        }
    }
}

fn line_clicked(
    graphics_context: &mut Vec<Draw>,
    game_grid: &mut Grid,
    is_column: bool,
    row: i32,
    column: i32,
) {
    if is_column {
        if row < 0
            || row >= game_grid.row_count() - 1
            || column < 0
            || column >= game_grid.column_count()
        {
            return;
        } // Click out of bounds.

        // Toggle a column.
        let wall = game_grid.get_wall(true, column, row);
        game_grid.set_wall_with_wall(&wall, !wall.set);

        // Update squares.
        if column > 0 {
            draw_square(
                graphics_context,
                (column - 1) as f32,
                row as f32,
                SQUARE_FILL[game_grid.get_wall_count(column - 1, row) as usize],
            );
        }
        if column < game_grid.column_count() - 1 {
            draw_square(
                graphics_context,
                column as f32,
                row as f32,
                SQUARE_FILL[game_grid.get_wall_count(column, row) as usize],
            );
        }
    } else {
        if row < 0
            || row >= game_grid.row_count()
            || column < 0
            || column >= game_grid.column_count() - 1
        {
            return;
        } // Click out of bounds.

        // Toggle a row.
        let wall = game_grid.get_wall(false, column, row);
        game_grid.set_wall_with_wall(&wall, !wall.set);

        // Update squares
        if row > 0 {
            draw_square(
                graphics_context,
                column as f32,
                (row - 1) as f32,
                SQUARE_FILL[game_grid.get_wall_count(column, row - 1) as usize],
            );
        }
        if row < game_grid.row_count() - 1 {
            draw_square(
                graphics_context,
                column as f32,
                row as f32,
                SQUARE_FILL[game_grid.get_wall_count(column, row) as usize],
            );
        }
    }

    // Update the stroke of all lines.
    redraw_lines(graphics_context, &game_grid);
}

fn main() {
    with_2d_graphics(|| {
        let mut game_grid = Grid::new(8, 10);

        let window_width: u64 = 800;
        let window_height: u64 = 600;
        let (canvas, events) = create_drawing_window_with_events(WindowProperties {
            title: BindRef::from(&String::from("Dots and Boxes Analysis")),
            size: BindRef::from(&(window_width, window_height)),
            fullscreen: BindRef::from(&false),
            has_decorations: BindRef::from(&true),
            mouse_pointer: BindRef::from(&MousePointer::SystemDefault),
        });

        let (canvas_transform, mut window_transform) = calculate_transform(
            window_width,
            window_height,
            game_grid.column_count(),
            game_grid.row_count(),
        );

        canvas.draw(|graphics_context| {
            redraw_all(graphics_context, canvas_transform, &game_grid);
        });

        executor::block_on(async move {
            let mut events = events;

            // Main event loop
            while let Some(event) = events.next().await {
                match event {
                    // Window resize
                    DrawEvent::Resize(width, height) => {
                        if width >= 1.0 && width >= 1.0 {
                            // Update the transform (only if the window has a valid size).
                            window_transform = calculate_transform(
                                width as u64,
                                height as u64,
                                game_grid.column_count(),
                                game_grid.row_count(),
                            )
                            .1;
                            canvas.draw(|graphics_context| {
                                redraw_all(graphics_context, canvas_transform, &game_grid);
                            });
                        }
                    }
                    DrawEvent::Pointer(PointerAction::ButtonDown, _id, state) => {
                        if state.buttons.contains(&Button::Left) {
                            // Process a left click action.

                            // Convert a window coordinate into a canvas coordinate (state.location_in_canvas is incorrect due to a bug).
                            let (x, y) = window_transform.transform_point(
                                state.location_in_window.0 as f32,
                                state.location_in_window.1 as f32,
                            );

                            let mut column = x as i32;
                            let column_remainder = x % 1.0;
                            let mut row = y as i32;
                            let row_remainder = y % 1.0;
                            let is_column = if column_remainder > row_remainder {
                                if 1.0 - column_remainder < row_remainder {
                                    column += 1; // column + 1
                                    true // column
                                } else {
                                    false // row
                                }
                            } else {
                                if 1.0 - row_remainder < column_remainder {
                                    row += 1; // row + 1
                                    false // row
                                } else {
                                    true // column
                                }
                            };

                            println!(
                                "Click at x: {}, y:{}, row_f: {x}, column_f: {y} on {} ({column}, {row})",
                                state.location_in_window.0, state.location_in_window.1, if is_column {"column"} else {"row"}
                            );

                            canvas.draw(|graphics_context| {
                                line_clicked(
                                    graphics_context,
                                    &mut game_grid,
                                    is_column,
                                    row,
                                    column,
                                );
                            })
                        }
                    }

                    // Ignore other events
                    _ => {}
                }
            }
        })
    });
}

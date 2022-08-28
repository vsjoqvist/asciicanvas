use ascii_canvas::{Figure, Game, Point};
use crossterm::terminal::size;

fn main() {
    let mut game = Game::new(size().unwrap()).unwrap();
    let test_figure = "aaaa\naaab";
    let test_figure_two = "hello 123";
    let test_figure = Figure::new(test_figure).unwrap();
    let test_figure_two = Figure::new(test_figure_two).unwrap();

    let _x = 0;
    let canvas = game.borrow_canvas_mut();
    {
        canvas.draw(&test_figure, Point { x: 0, y: 0 });
        canvas.draw(&test_figure_two, Point { x: 0, y: 0 });
        canvas.draw_canvas(&Point { x: 0, y: 10 });
        dbg!(&canvas.canvas);
    }
}
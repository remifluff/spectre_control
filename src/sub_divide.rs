use nannou::prelude::*;

pub trait SubdivideExt {
    fn divide_columns(&self, count: u32) -> Vec<Rect>;
    fn divide_rows(&self, count: u32) -> Vec<Rect>;
    fn divide_rows_cols(&self, rows: u32, columns: u32) -> Vec<Vec<Rect>> {
        self.divide_rows(rows).iter().map(|row| row.divide_columns(columns)).collect()
    }
}
impl SubdivideExt for geom::Rect {
    fn divide_rows(&self, subdivisions: u32) -> Vec<Self> {
        let parent = self;
        //create a cell the same size and the final cell
        //divide it by the amount of subdevistions on the chosen axis

        let cell_wh = Rect::from_w_h(parent.w(), parent.h() / subdivisions as f32);

        // loop over the subdeviosions to create all the cells
        (0..subdivisions)
            .map(|i| {
                let mut position = vec2(0.0, i as f32) * cell_wh.wh();

                //move it to 0,0
                position -= vec2(0.0, parent.h() / 2.0);
                position += vec2(0.0, cell_wh.h() / 2.0);

                //invert so rows index from left to right
                position *= vec2(1.0, -1.0);

                //move it to the postion of the parent rect
                position += parent.xy();

                cell_wh.shift(position)
            })
            .collect()
    }
    fn divide_columns(&self, subdivisions: u32) -> Vec<Self> {
        let parent = self;
        //create a cell the same size and the final cell
        //divide it by the amount of subdevistions on the chosen axis

        let cell_wh = Rect::from_w_h(parent.w() / subdivisions as f32, parent.h());

        // loop over the subdeviosions to create all the cells
        (0..subdivisions)
            .map(|i| {
                let mut position = vec2(i as f32, 0.0) * cell_wh.wh();

                //move it to 0,0
                position -= vec2(parent.w() / 2.0, 0.0);
                position += vec2(cell_wh.w() / 2.0, 0.0);

                //move it to the postion of the parent rect
                position += parent.xy();

                cell_wh.shift(position)
            })
            .collect()
    }
}

use pyo3::prelude::*;

use lophat::columns::{Column, VecColumn};
// TODO: How to notice when map is an inclusion and do less work?

pub fn extend_telescope(
    mut telescope: Vec<VecColumn>,
    codomain_matrix: Vec<VecColumn>,
    map: Vec<VecColumn>,
    domain_range: (usize, usize),
) -> Vec<VecColumn> {
    let (domain_start, domain_end) = domain_range;
    let codomain_size = codomain_matrix.len();
    let init_telescope_size = telescope.len();

    // Add in the codomain column
    // The boundaries need to be adjusted because they are in new idxs

    for col in codomain_matrix {
        let new_boundary = col
            .entries()
            .map(|row_idx| row_idx + init_telescope_size)
            .collect();
        let new_col = VecColumn::from((col.dimension(), new_boundary));
        telescope.push(new_col);
    }

    // Add in the columns from domain cells in shifted dimension
    // The boundary has to include the normal boundary (after shifting)
    // As well as the original domain cell and the target codomain cells

    // This is the first index where we will the shifted domain columns
    let domain_shift_start = init_telescope_size + codomain_size;
    // This is how much the domain_shift idexes get shifted up by
    let domain_shift_idx_diff = domain_shift_start - domain_start;
    // This is the first index where we see codomain columns
    let codomain_start = init_telescope_size;

    for domain_idx in domain_start..domain_end {
        // The shifted column has one end touching the original domain cell
        let mut new_boundary = vec![domain_idx];
        let old_boundary_post_shift = telescope
            .get(domain_idx)
            .unwrap()
            .entries()
            .map(|row_idx| row_idx + domain_shift_idx_diff);
        let image_bdry = map
            .get(domain_idx)
            .unwrap()
            .entries()
            .map(|row_idx| row_idx + codomain_start);
        new_boundary.extend(image_bdry);
        new_boundary.extend(old_boundary_post_shift);
        let old_col = telescope.get(domain_idx).unwrap();
        let new_col = VecColumn::from((old_col.dimension() + 1, new_boundary));
        telescope.push(new_col);
    }

    telescope
}

pub fn build_telescope(
    matrices: Vec<Vec<VecColumn>>,
    maps: Vec<Vec<VecColumn>>,
) -> Vec<(usize, VecColumn)> {
    let mut matrix_iter = matrices.into_iter();
    let mut telescope = matrix_iter.next().unwrap();
    let mut domain_range = (0, telescope.len());

    let mut filtration_vector = vec![0; telescope.len()];
    let mut next_f = 1;

    for (codomain_matrix, map) in matrix_iter.zip(maps) {
        // Store lengths
        let init_telescope_len = telescope.len();
        let codomain_size = codomain_matrix.len();

        // Extend telescope
        telescope = extend_telescope(telescope, codomain_matrix, map, domain_range);

        // Write down range where domain of next matrix is stored
        // Codomain columns get added first
        domain_range = (init_telescope_len, init_telescope_len + codomain_size);
        // Extend the filtration vector
        let final_telescope_len = telescope.len();
        filtration_vector.extend((init_telescope_len..final_telescope_len).map(|_i| next_f));

        next_f = next_f + 1
    }

    filtration_vector
        .into_iter()
        .zip(telescope.into_iter())
        .collect()
}

type VecColumnPy = (usize, Vec<usize>);

#[pyfunction]
#[pyo3(name = "build_telescope")]
fn build_telescope_py(
    matrices: Vec<Vec<VecColumnPy>>,
    maps: Vec<Vec<VecColumnPy>>,
) -> Vec<(usize, VecColumnPy)> {
    let matrices = matrices
        .into_iter()
        .map(|matrix| matrix.into_iter().map(|col| col.into()).collect())
        .collect();
    let maps = maps
        .into_iter()
        .map(|map_matrix| map_matrix.into_iter().map(|col| col.into()).collect())
        .collect();
    let telescope = build_telescope(matrices, maps);

    telescope
        .into_iter()
        .map(|(f_val, col)| {
            let col_py = (col.dimension(), col.entries().collect());
            (f_val, col_py)
        })
        .collect()
}

/// A Python module implemented in Rust.
#[pymodule]
fn phubble(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(build_telescope_py, m)?)?;
    Ok(())
}

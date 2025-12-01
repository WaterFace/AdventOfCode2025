pub fn variance(pop: &[f32]) -> f32 {
    let mean = pop.iter().sum::<f32>() / pop.len() as f32;
    pop.iter().map(|s| (s - mean) * (s - mean)).sum::<f32>() / pop.len() as f32
}

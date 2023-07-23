use rand::{rngs::ThreadRng, Rng};

pub fn new(
    rng: &mut ThreadRng,
    words: &mut Vec<String>,
    num_words: usize,
    separator: &str,
) -> String {
    if words.len() < num_words {
        eprintln!(
            "Your dictionary only has {} suitable words, but you asked for {} words.",
            words.len(),
            num_words
        );
        return "".to_string();
    }

    (0..num_words).for_each(|i| {
        let j = rng.gen_range(i..words.len());
        words.swap(i, j)
    });

    (0..num_words)
        .map(|i| words[i].to_owned())
        .collect::<Vec<String>>()
        .join(separator)
}

mod test {
    #[test]
    fn chi_squared() {
        use crate::{passphrase, words};
        use statrs::distribution::{ChiSquared, ContinuousCDF};
        use std::collections::HashMap;

        let n = 4;
        let n_fact = 24;
        // this test file has n = 4 words, which can have 24 permutations
        let words = words::list(Some("src/fixtures/test")).unwrap();

        let trials = 1_200_000;
        let mut rng = rand::thread_rng();

        let mut histogram: HashMap<String, u32> = HashMap::new();
        (1..trials).for_each(|_| {
            let mut words = words.clone();
            let s = passphrase::new(&mut rng, &mut words, n, " ");
            *histogram.entry(s).or_insert(0) += 1;
        });

        assert_eq!(histogram.len(), n_fact);

        let expected_frequency = trials as f64 / n_fact as f64;
        let chi_squared_stat: f64 = histogram
            .iter()
            .map(|(_, v)| (*v as f64 - expected_frequency).powi(2) / expected_frequency)
            .sum();

        // degrees of freedom = (number of rows - 1) * (number of columns - 1)
        let df = ((2 - 1) * (24 - 1)) as f64;
        let dist = ChiSquared::new(df).unwrap();
        let p = 1.0 - dist.cdf(chi_squared_stat);

        eprintln!("χ^2: {}", chi_squared_stat);
        eprintln!("p: {}", p);

        // the p-value should be greater than 0.05 so that we can't reject the null hypothesis
        // if we can reject the null hypothesis, then the passphrase generator is not uniform
        assert_eq!(p > 0.05, true);
    }
}

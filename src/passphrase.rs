use rand::{rngs::ThreadRng, Rng};

pub fn new(rng: &mut ThreadRng, words: &mut [String], num_words: usize, separator: &str) -> String {
    if words.len() < num_words {
        eprintln!(
            "Your dictionary only has {} suitable words, but you asked for {} words.",
            words.len(),
            num_words
        );
        return String::new();
    }

    (0..num_words).for_each(|i| {
        let j = rng.gen_range(i..words.len());
        words.swap(i, j);
    });

    (0..num_words)
        .map(|i| words[i].clone())
        .collect::<Vec<String>>()
        .join(separator)
}

mod test {
    #[test]
    // Uses [Pearson's chi-squared test](https://en.wikipedia.org/wiki/Chi-squared_test#Pearson's_chi-squared_test)
    // to test that the passphrases are uniformly distributed.
    fn chi_squared() {
        use crate::{passphrase, words};
        use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
        use statrs::distribution::{ChiSquared, ContinuousCDF};
        use std::collections::HashMap;

        // This test file has W = 4 words, which can have 24 permutations
        const W: usize = 4;
        const W_FACTORIAL: usize = 24;
        const N: usize = 12_000_000; // number of samples

        let words = words::list(Some("src/fixtures/test")).unwrap();

        let histogram = Vec::from_iter(0..N)
            .par_iter()
            .fold_chunks(
                N / std::thread::available_parallelism().unwrap(),
                || HashMap::new(),
                |mut acc, _| {
                    let mut rng = rand::thread_rng();
                    let mut words = words.clone();
                    let s = passphrase::new(&mut rng, &mut words, W, " ");
                    *acc.entry(s).or_insert(0) += 1 as usize;
                    acc
                },
            )
            .collect::<Vec<HashMap<String, usize>>>()
            .iter()
            .fold(HashMap::new(), |mut acc, h| {
                h.iter().for_each(|(k, v)| {
                    *acc.entry(k.to_owned()).or_insert(0) += v;
                });
                acc
            });

        assert_eq!(histogram.values().sum::<usize>(), N, "missing samples");

        // There should be at most W! different passphrases. If, by chance, some of them are not
        // generated, then the chi-squared test is highly unlikely to conclude that they are
        // uniformly distributed.
        assert_eq!(W_FACTORIAL, histogram.len(), "missing a permutation");

        let expected_frequency = N as f64 / W_FACTORIAL as f64;
        let chi_squared_stat: f64 = histogram
            .values()
            .map(|v| (*v as f64 - expected_frequency).powi(2) / expected_frequency)
            .sum();

        // Since the number in any permutation is determined by the number in all the others,
        // degrees of freedom = number of permutations - 1
        const DF: f64 = (W_FACTORIAL - 1) as f64;
        let dist = ChiSquared::new(DF).unwrap();

        // The p-value is the area under the chi-squared pdf to the right of the chi_squared_stat
        let p = 1.0 - dist.cdf(chi_squared_stat);

        // The p-value should be greater than 0.05 so that we can't reject the null hypothesis that
        // the values are from a uniform distribution.
        // If we can reject the null hypothesis, then the passphrase generator may not be uniform.
        assert!(
            p > 0.05,
            "passphrase may not be uniformly random. (p = {} <= 0.05, χ^2 = {}).",
            p,
            chi_squared_stat,
        );
    }
}

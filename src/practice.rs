use rand::prelude::IndexedRandom;


pub const TYPING_LEVELS: [(&str, &[char]); 32] = [
    ("new: f & j", &['f', 'j']),
    ("new: d & k", &['d', 'k']),
    ("repetition: f, j, d, k", &['f', 'j', 'd', 'k']),
    ("new: s & l", &['s', 'l']),
    ("repetition: home row 1", &['f', 'j', 'd', 'k', 's', 'l']),
    ("new: a & ;", &['a', ';']),
    ("repetition: home row 2", &['f', 'j', 'd', 'k', 's', 'l', 'a', ';']),
    ("new: g & h", &['g', 'h']),
    ("repetition: home row full", &['f', 'j', 'd', 'k', 's', 'l', 'a', ';', 'g', 'h']),
    ("new: r & u", &['r', 'u']),
    ("repetition: add r & u", &['f', 'j', 'd', 'k', 's', 'l', 'a', ';', 'g', 'h', 'r', 'u']),
    ("new: t & y", &['t', 'y']),
    ("repetition: left-right pairs", &['r', 'u', 't', 'y', 'g', 'h']),
    ("new: e & i", &['e', 'i']),
    ("repetition: stretch row 1", &['r', 'u', 't', 'y', 'e', 'i']),
    ("new: w & o", &['w', 'o']),
    ("new: q & p", &['q', 'p']),
    ("repetition: top row left-right", &['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p']),
    ("new: v, b, n", &['v', 'b', 'n']),
    ("new: c & m", &['c', 'm']),
    ("repetition: bottom row left-right", &['z', 'x', 'c', 'v', 'b', 'n', 'm']),
    ("new: x & ,", &['x', ',']),
    ("new: z & . & /", &['z', '.', '/']),
    ("repetition: full lowercase", &[
        'a','b','c','d','e','f','g','h','i','j',
        'k','l','m','n','o','p','q','r','s','t',
        'u','v','w','x','y','z'
    ]),
    ("new: numbers row", &['1','2','3','4','5','6','7','8','9','0']),
    ("repetition: letters + numbers", &[
        'a','s','d','f','j','k','l',';', '1','2','3','4','5'
    ]),
    ("new: symbols row 1", &['-', '=', '[', ']', '\'', ';', ',', '.', '/', '`']),
    ("new: shifted: !@#$", &['!', '@', '#', '$']),
    ("new: shifted: %^&*()", &['%', '^', '&', '*', '(', ')']),
    ("repetition: shift practice", &['!', '@', '#', '$', '%', '^', '&', '*', '(', ')']),
    ("new: shifted: symbols", &['_', '+', '{', '}', ':', '"', '<', '>', '?', '~']),
    ("repetition: all punctuation", &[
        '.', ',', ':', ';', '!', '?', '\'', '"', '-', '_', '(', ')',
        '[', ']', '{', '}', '/', '\\', '`', '~'
    ]),
];


pub fn create_words(chars: &[char], word_number: usize) -> String {
    let mut reference = String::new();
    for i in 0..word_number {
        let word_length = rand::random::<u16>() % 5 + 2;
        let word: String = (0..word_length)
            .map(|i| *chars.choose(&mut rand::rng()).unwrap())
            .collect();
        reference.push_str(&word);
        if i != word_number - 1 {
            reference.push(' ');
        }
    }
    return reference;
}

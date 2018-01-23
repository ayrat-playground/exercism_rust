use std::collections::HashMap;

pub struct RNA {
    nucleotides: Vec<char>
}

pub struct DNA {
    nucleotides: Vec<char>
}

const RNA_NUCLEOTIDES: &'static [char] = &['C', 'G', 'A', 'U'];
const DNA_NUCLEOTIDES: &'static [char] = &['G', 'C', 'T', 'A'];

impl RNA {
    pub fn new(string: &str) -> Result<Self, &'static str> {
        let nucleotides = string_to_nucleotides(string, RNA_NUCLEOTIDES)?;

        Ok(RNA { nucleotides })
    }

    pub fn to_dna(val: Result<RNA, &'static str>) -> Result<DNA, &'static str> {
        let hash: HashMap<char, char> = [
            ('G','C'), ('C','G'), ('T','A'), ('A','U')
        ].iter().cloned().collect();

        let nucleotides = val?.nucleotides.iter().map(|ch| *hash.get(&ch).unwrap()).collect::<Vec<char>>();

        Ok(DNA{ nucleotides })
    }
}

impl DNA {
    pub fn new(string: &str) -> Result<Self, &'static str> {
        let nucleotides = string_to_nucleotides(string, DNA_NUCLEOTIDES)?;

        Ok(DNA { nucleotides })
    }


    pub fn to_rna(val: Result<DNA, &'static str>) -> Result<RNA, &'static str>  {
        let hash: HashMap<char, char> = [
            ('C','G'), ('G','C'), ('A','T'), ('U','A')
        ].iter().cloned().collect();

        let nucleotides = val?.nucleotides.iter().map(|ch| *hash.get(&ch).unwrap()).collect::<Vec<char>>();

        Ok(RNA{ nucleotides })
    }
}

fn string_to_nucleotides(string: &str, nucleotides: &[char]) -> Result<Vec<char>, &'static str> {
    if string.chars().all(|ch| nucleotides.iter().any(|n| *n == ch)) {
        return Err("oops")
    }

    Ok(string.chars().collect::<Vec<char>>())
}

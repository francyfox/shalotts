use crc32fast::Hasher;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegistryItem {
    pub id: String,
    pub label: String,
    pub group: String,
    pub target: Vec<String>,
    pub source: String,
    pub template: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub metadata: HashMap<String, String>,
    pub registry: Vec<RegistryItem>,
}

pub struct BioDecoder {
    config: Config,
    promoter: String,
}

impl BioDecoder {
    const MAP: [char; 4] = ['A', 'C', 'G', 'T'];

    pub fn new(json_content: &str) -> Result<Self, String> {
        let config: Config =
            serde_json::from_str(json_content).map_err(|e| format!("JSON Error: {}", e))?;

        let id = config.metadata.get("id").ok_or("Metadata ID not found")?;

        let promoter = Self::generate_dna_hash(id, 4);
        Ok(Self { config, promoter })
    }

    /// Генерирует детерминированный ДНК-код на основе строки
    fn generate_dna_hash(input: &str, length: usize) -> String {
        let mut hasher = Hasher::new();
        hasher.update(input.as_bytes());
        let hash = hasher.finalize();

        let mut dna = String::new();
        for i in 0..length {
            let bits = (hash >> (i * 2)) & 0b11;
            dna.push(Self::MAP[bits as usize]);
        }
        dna
    }

    /// Определяет позицию (индекс бита) для конкретного пакета
    fn get_feature_bit_pos(&self, feature_id: &str) -> u32 {
        let mut hasher = Hasher::new();
        hasher.update(feature_id.as_bytes());
        // Ограничиваем позицию, например, 64 битами (32 символа ДНК)
        hasher.finalize() % 64
    }

    /// Декодирует сид и возвращает список элементов для сборки
    pub fn decode_seed(&self, seed: &str) -> Vec<RegistryItem> {
        // Проверяем, наш ли это локус (начало сида == промоутер)
        if !seed.starts_with(&self.promoter) {
            return vec![];
        }

        // Убираем промоутер и терминатор AAA
        let payload = seed
            .trim_start_matches(&self.promoter)
            .trim_end_matches("AAA");

        // Создаем битовую маску из ДНК
        let mut active_bits = std::collections::HashSet::new();
        for (idx, nucleotide) in payload.chars().enumerate() {
            let bits = match nucleotide {
                'A' => 0b00,
                'C' => 0b01,
                'G' => 0b10,
                'T' => 0b11,
                _ => continue,
            };
            let base_pos = (idx * 2) as u32;
            if bits & 0b10 != 0 {
                active_bits.insert(base_pos);
            }
            if bits & 0b01 != 0 {
                active_bits.insert(base_pos + 1);
            }
        }

        // Сопоставляем биты с реестром через хеширование их ID
        self.config
            .registry
            .iter()
            .filter(|item| {
                let pos = self.get_feature_bit_pos(&item.id);
                active_bits.contains(&pos)
            })
            .cloned()
            .collect()
    }
}

import crc32 from "crc/crc32";
import type { ConfigRoot } from "~/types/config.types";

/**
 * BioEncoder — трансформации системных идентификаторов в ДНК-последовательности.
 * * @description
 * Использует алгоритм CRC32 для проекции строковых ID на 64-битную генетическую карту.
 * * @param {Config} config - Конфигурация, содержащая метаданные проекта и реестр доступных айтемов.
 * @returns {Object} Набор методов для генерации ДНК-хеша, энкодинга выбранных ID и их декодинга из сида.
 */
export const BioEncoder = (config: ConfigRoot) => {
	const MAP = ["A", "C", "G", "T"] as const;
	type Nucleotide = (typeof MAP)[number];

	const registryLen = config.registry.length;
	const registryPositions = new Int8Array(registryLen);
	const registryIds: string[] = new Array(registryLen);

	for (let i = 0; i < registryLen; i++) {
		const item = config.registry[i]!;
		registryIds[i] = item.id;
		registryPositions[i] = (crc32(item.id) >>> 0) % 64;
	}

	const getPos = (id: string): number => (crc32(id) >>> 0) % 64;
	const generateDnaHash = (input: string, length: number): string => {
		const hash = crc32(input) >>> 0;
		let dna = "";
		for (let i = 0; i < length; i++) {
			dna += MAP[(hash >>> (i * 2)) & 0b11];
		}
		return dna;
	};

	const promoter = generateDnaHash(config.metadata.id, 4);

	return {
		encode: (selectedIds: string[]): string => {
			const activeBits = new Uint8Array(64);
			let maxBit = -1;

			for (let i = 0; i < selectedIds.length; i++) {
				const id = selectedIds[i];
				if (id === undefined) continue;
				const pos = getPos(id);
				activeBits[pos] = 1;
				if (pos > maxBit) maxBit = pos;
			}

			if (maxBit === -1) return `${promoter}AAA`;

			let payload = "";
			const dnaLength = Math.ceil((maxBit + 1) / 2);
			for (let i = 0; i < dnaLength; i++) {
				const val =
					((activeBits[i * 2] || 0) << 1) | (activeBits[i * 2 + 1] || 0);
				payload += MAP[val as 0 | 1 | 2 | 3];
			}
			return `${promoter}${payload}AAA`;
		},

		decode: (seed: string): string[] => {
			if (!seed.startsWith(promoter)) return [];
			const payload = seed.slice(promoter.length, -3);
			const bits = new Uint8Array(64);

			for (let i = 0; i < payload.length; i++) {
				const char = payload[i] as Nucleotide;
				const val = MAP.indexOf(char);
				if (val === -1) continue;
				bits[i * 2] = val & 0b10 ? 1 : 0;
				bits[i * 2 + 1] = val & 0b01 ? 1 : 0;
			}

			const result: string[] = [];
			for (let i = 0; i < registryLen; i++) {
				const pos = registryPositions[i]!;
				if (bits[pos]) result.push(registryIds[i]!);
			}
			return result;
		},
	};
};

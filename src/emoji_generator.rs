use rand::Rng;

/// A module for generating random sets of emojis
pub struct EmojiGenerator {
    emojis: Vec<&'static str>,
}

impl EmojiGenerator {
    /// Create a new EmojiGenerator with a default set of emojis
    pub fn new() -> Self {
        Self {
            emojis: vec![
                // Faces
                "😀", "😃", "😄", "😁", "😆", "😅", "🤣", "😂", "🙂", "🙃", "😉", "😊", "😇", "🇧🇩",
                "😈", "🤩", "✅", "🤖", "👨🏾‍💻", "🤯", "😋", "😛", "😜", "🤪", "😝", "🤑", "🤗", "🤭",
                "🤫", "🤔", "🤐", "🤨", "😐", "😑", "😶", "😏", "😒", "🙄", "😬", "🤥",
                // Animals
                "🐶", "🐱", "🐭", "🐹", "🐰", "🦊", "🐻", "🐼", "🐨", "🐯", "🦁", "🐮", "🐷", "🐸",
                "🐵", "🙈", "🙉", "🙊", "🐒", "🐔", "🐧", "🐦", "🐤", "🐣", "🐥", "🦆", "🦅", "🦉",
                "🦇", "🐺", "🐗", "🐴", "🦄", "🐝", "🐛", "🦋", "🐌", "🐞", "🐜", "🦟",
                // Food
                "🍎", "🍐", "🍊", "🍋", "🍌", "🍉", "🍇", "🍓", "🫐", "🍈", "🍒", "🍑", "🥭", "🍍",
                "🥥", "🥝", "🍅", "🍆", "🥑", "🥦", "🥬", "🥒", "🌶️", "🫑", "🌽", "🥕", "🫒", "🧄",
                "🧅", "🥔", "🍞", "🥐", "🥖", "🫓", "🥨", "🥯", "🥞", "🧇", "🧀", "🍖",
                // Activities
                "⚽", "🏀", "🏈", "⚾", "🥎", "🎾", "🏐", "🏉", "🥏", "🎱", "🪀", "🏓", "🏸", "🏒",
                "🏑", "🥍", "🏏", "🪃", "🥅", "⛳", "🪁", "🏹", "🎣", "🤿", "🥊", "🥋", "🎽", "🛹",
                "🛷", "⛸️", // Objects
                "⌚", "📱", "📲", "💻", "⌨️", "🖥️", "🖨️", "🖱️", "🖲️", "🕹️", "🗜️", "💽", "💾", "💿",
                "📀", "📼", "📷", "📸", "📹", "🎥", "📞", "☎️", "📟", "📠", "📺", "📻", "🎙️", "🎚️",
                "🎛️", "🧭", // Weather
                "☀️", "🌤️", "⛅", "🌥️", "☁️", "🌦️", "🌧️", "⛈️", "🌩️", "🌨️", "❄️", "☃️", "⛄", "🌬️",
                "💨", "🌪️", "🌫️", "🌊", "💧", "💦", // Nature
                "🌱", "🌿", "☘️", "🍀", "🎍", "🎋", "🍃", "🍂", "🍁", "🌾", "🌵", "🌴", "🌳", "🌲",
                // Places
                "🏠", "🏡", "🏢", "🏣", "🏤", "🏥", "🏦", "🏨", "🏩", "🏪", "🏫", "🏬", "🏭", "🏯",
                "🏰", "🏗️", "🏘️", "🏚️", "🌇", "🌆", "🌃", "🌉", "🌌", "🗼", "🗽", "⛪", "🕌",
                // Transportation
                "🚗", "🚕", "🚙", "🚌", "🚎", "🏎️", "🚓", "🚑", "🚒", "🚐", "🚚", "🚛", "🚜",
                "🚲", "🛴", "🛵", "🏍️", "🚨", "🚔", "🚍", "🚘", "🚖", "🚡", "🚠", "🚟", "🚃",
                "🚋", "🚝", "🚄", "🚅", "🚈", "🚂", "🚆", "🚇", "🚊", "✈️", "🛩️", "🛫", "🛬",
                // Technology
                "🛰️", "🚀", "🛸", "💺", "🚁", "🚤", "⛵", "🛳️", "⚓", "⛽", "🚧", "🚦", "🚥",
                "🛑", "🚏", "🗺️", "🗿", "🏴‍☠️", "🏳️‍🌈", "🏳️‍⚧️", "🏁", "🚩", "🎌", "🏳️",
            ],
        }
    }

    /// Generate a random set of unique emojis (no duplicates)
    pub fn generate(&self, count: usize) -> Vec<String> {
        let mut rng = rand::rng();
        let max_count = std::cmp::min(count, self.emojis.len());

        let mut indices: Vec<usize> = (0..self.emojis.len()).collect();

        // Fisher-Yates shuffle
        for i in (1..indices.len()).rev() {
            let j = rng.random_range(0..=i);
            indices.swap(i, j);
        }

        indices
            .into_iter()
            .take(max_count)
            .map(|i| self.emojis[i].to_string())
            .collect()
    }
}

impl Default for EmojiGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_set() {
        let generator = EmojiGenerator::new();
        let emojis = generator.generate(5);
        assert_eq!(emojis.len(), 5);
    }
}

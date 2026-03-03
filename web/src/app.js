// Seed Phrase Auto Recovery - Web Interface
// This is a client-side implementation for demonstration
// In production, this would call the Rust backend via WebAssembly or API

class SeedRecovery {
    constructor() {
        this.wordlist = this.loadWordlist();
        this.isRecovering = false;
        this.startTime = null;
        this.attempts = 0;
        
        this.initEventListeners();
    }

    initEventListeners() {
        document.getElementById('recoverBtn').addEventListener('click', () => this.startRecovery());
        document.getElementById('validateBtn').addEventListener('click', () => this.validatePhrase());
        document.getElementById('suggestBtn').addEventListener('click', () => this.suggestWords());
    }

    loadWordlist() {
        // BIP39 English wordlist (subset for demo)
        return [
            "abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract",
            "absurd", "abuse", "access", "accident", "account", "accuse", "achieve", "acid",
            "acoustic", "acquire", "across", "act", "action", "actor", "actress", "actual",
            "adapt", "add", "addict", "address", "adjust", "admit", "adult", "advance",
            // ... (in production, include all 2048 words)
        ];
    }

    async startRecovery() {
        if (this.isRecovering) {
            alert('Recovery already in progress!');
            return;
        }

        const phrase = document.getElementById('seedPhrase').value.trim();
        const targetAddress = document.getElementById('targetAddress').value.trim();
        const cryptoType = document.getElementById('cryptoType').value;

        if (!phrase) {
            alert('Please enter a seed phrase');
            return;
        }

        const words = phrase.split(/\s+/);
        const missingCount = words.filter(w => w === '???' || w === '?').length;

        if (missingCount === 0) {
            alert('No missing words found. Use ??? to mark unknown positions');
            return;
        }

        if (missingCount > 3) {
            alert('Too many missing words. Maximum 3 for reasonable recovery time in browser');
            return;
        }

        this.isRecovering = true;
        this.startTime = Date.now();
        this.attempts = 0;

        this.showProgress();
        this.updateProgress('Initializing recovery...', 0);

        try {
            const result = await this.recover(words, targetAddress, cryptoType);
            this.showResult(result);
        } catch (error) {
            this.showError(error.message);
        } finally {
            this.isRecovering = false;
        }
    }

    async recover(words, targetAddress, cryptoType) {
        return new Promise((resolve, reject) => {
            // Simulate recovery process
            // In production, this would call Rust backend via WebAssembly
            
            const missingPositions = [];
            words.forEach((word, idx) => {
                if (word === '???' || word === '?') {
                    missingPositions.push(idx);
                }
            });

            const totalCombinations = Math.pow(this.wordlist.length, missingPositions.length);
            let found = false;

            setTimeout(() => {
                // Simulate finding a result
                const recoveredWords = [...words];
                missingPositions.forEach((pos, idx) => {
                    recoveredWords[pos] = this.wordlist[idx % this.wordlist.length];
                });

                resolve({
                    phrase: recoveredWords.join(' '),
                    address: this.generateDummyAddress(cryptoType),
                    attempts: Math.floor(Math.random() * 10000),
                    time: (Date.now() - this.startTime) / 1000
                });
            }, 2000);
        });
    }

    validatePhrase() {
        const phrase = document.getElementById('seedPhrase').value.trim();
        
        if (!phrase) {
            alert('Please enter a seed phrase');
            return;
        }

        const words = phrase.split(/\s+/);
        const expectedLengths = [12, 15, 18, 21, 24];

        if (!expectedLengths.includes(words.length)) {
            this.showResult({
                success: false,
                message: `Invalid word count: ${words.length}. Expected: 12, 15, 18, 21, or 24 words`
            });
            return;
        }

        // Check if words are in wordlist
        const invalidWords = words.filter(w => !this.wordlist.includes(w) && w !== '???');
        
        if (invalidWords.length > 0) {
            this.showResult({
                success: false,
                message: `Invalid words found: ${invalidWords.join(', ')}`
            });
            return;
        }

        this.showResult({
            success: true,
            message: '✓ Phrase structure is valid (checksum validation requires backend)'
        });
    }

    suggestWords() {
        const phrase = document.getElementById('seedPhrase').value.trim();
        const words = phrase.split(/\s+/);
        
        // Find words that might be typos
        const suggestions = [];
        
        words.forEach((word, idx) => {
            if (word !== '???' && !this.wordlist.includes(word)) {
                const similar = this.findSimilarWords(word, 2);
                if (similar.length > 0) {
                    suggestions.push({
                        position: idx + 1,
                        original: word,
                        suggestions: similar.slice(0, 5)
                    });
                }
            }
        });

        if (suggestions.length === 0) {
            this.showResult({
                success: true,
                message: 'No potential typos found or all words are valid'
            });
            return;
        }

        let html = '<div class="suggestions">';
        suggestions.forEach(s => {
            html += `<div class="suggestion-item">
                <strong>Position ${s.position}: "${s.original}"</strong>
                <p>Did you mean: ${s.suggestions.join(', ')}?</p>
            </div>`;
        });
        html += '</div>';

        this.showResult({
            success: true,
            message: 'Suggestions found:',
            html: html
        });
    }

    findSimilarWords(word, maxDistance) {
        return this.wordlist
            .filter(w => this.levenshteinDistance(word.toLowerCase(), w.toLowerCase()) <= maxDistance)
            .sort((a, b) => 
                this.levenshteinDistance(word.toLowerCase(), a.toLowerCase()) - 
                this.levenshteinDistance(word.toLowerCase(), b.toLowerCase())
            );
    }

    levenshteinDistance(str1, str2) {
        const len1 = str1.length;
        const len2 = str2.length;
        const matrix = Array(len1 + 1).fill(null).map(() => Array(len2 + 1).fill(0));

        for (let i = 0; i <= len1; i++) matrix[i][0] = i;
        for (let j = 0; j <= len2; j++) matrix[0][j] = j;

        for (let i = 1; i <= len1; i++) {
            for (let j = 1; j <= len2; j++) {
                const cost = str1[i - 1] === str2[j - 1] ? 0 : 1;
                matrix[i][j] = Math.min(
                    matrix[i - 1][j] + 1,
                    matrix[i][j - 1] + 1,
                    matrix[i - 1][j - 1] + cost
                );
            }
        }

        return matrix[len1][len2];
    }

    showProgress() {
        document.getElementById('progressSection').style.display = 'block';
        document.getElementById('resultSection').style.display = 'none';
        
        const updateInterval = setInterval(() => {
            if (!this.isRecovering) {
                clearInterval(updateInterval);
                return;
            }

            this.attempts += Math.floor(Math.random() * 1000);
            const elapsed = (Date.now() - this.startTime) / 1000;
            const speed = Math.floor(this.attempts / elapsed);

            document.getElementById('attempts').textContent = this.attempts.toLocaleString();
            document.getElementById('timeElapsed').textContent = `${elapsed.toFixed(1)}s`;
            document.getElementById('speed').textContent = `${speed.toLocaleString()}/s`;
        }, 100);
    }

    updateProgress(text, percentage) {
        document.getElementById('progressText').textContent = text;
        document.getElementById('progressPercentage').textContent = `${percentage}%`;
        document.getElementById('progressFill').style.width = `${percentage}%`;
    }

    showResult(result) {
        document.getElementById('progressSection').style.display = 'none';
        document.getElementById('resultSection').style.display = 'block';
        
        let html = '';
        
        if (result.phrase) {
            html = `
                <div class="result-success">
                    <h3>✅ Recovery Successful!</h3>
                    <div class="result-field">
                        <label>Recovered Phrase:</label>
                        <div class="phrase-display">${result.phrase}</div>
                        <button onclick="navigator.clipboard.writeText('${result.phrase}')">Copy</button>
                    </div>
                    <div class="result-field">
                        <label>Derived Address:</label>
                        <code>${result.address}</code>
                    </div>
                    <div class="result-stats">
                        <span>Attempts: <strong>${result.attempts.toLocaleString()}</strong></span>
                        <span>Time: <strong>${result.time.toFixed(2)}s</strong></span>
                    </div>
                </div>
            `;
        } else if (result.html) {
            html = `<div class="result-${result.success ? 'success' : 'error'}">
                <p>${result.message}</p>
                ${result.html}
            </div>`;
        } else {
            html = `<div class="result-${result.success ? 'success' : 'error'}">
                <p>${result.message}</p>
            </div>`;
        }
        
        document.getElementById('resultContent').innerHTML = html;
    }

    showError(message) {
        this.showResult({
            success: false,
            message: `❌ Error: ${message}`
        });
    }

    generateDummyAddress(cryptoType) {
        // Generate dummy address for demo
        if (cryptoType === 'bitcoin') {
            return '1' + this.randomHex(33);
        } else {
            return '0x' + this.randomHex(40);
        }
    }

    randomHex(length) {
        let result = '';
        const chars = '0123456789abcdef';
        for (let i = 0; i < length; i++) {
            result += chars.charAt(Math.floor(Math.random() * chars.length));
        }
        return result;
    }
}

// Initialize app
document.addEventListener('DOMContentLoaded', () => {
    new SeedRecovery();
});

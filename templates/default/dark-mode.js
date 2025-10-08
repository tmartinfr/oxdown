const darkModeToggle = document.getElementById('darkModeToggle');
const body = document.body;

// Load saved preference or default to dark
const savedTheme = localStorage.getItem('theme');
if (savedTheme === 'light') {
    body.classList.remove('dark-mode');
    darkModeToggle.textContent = '🌙';
} else {
    body.classList.add('dark-mode');
    darkModeToggle.textContent = '☀️';
}

// Toggle dark mode
darkModeToggle.addEventListener('click', () => {
    body.classList.toggle('dark-mode');

    if (body.classList.contains('dark-mode')) {
        darkModeToggle.textContent = '☀️';
        localStorage.setItem('theme', 'dark');
    } else {
        darkModeToggle.textContent = '🌙';
        localStorage.setItem('theme', 'light');
    }
});

const lightModeToggle = document.getElementById("lightModeToggle");
const body = document.body;
const highlightTheme = document.getElementById("highlight-theme");

// Load saved preference or default to dark
const savedTheme = localStorage.getItem("theme");
if (savedTheme === "light") {
  body.classList.add("light-mode");
  lightModeToggle.textContent = "🌙";
  if (highlightTheme) {
    highlightTheme.href = "/css/github.min.css";
  }
} else {
  body.classList.remove("light-mode");
  lightModeToggle.textContent = "☀️";
  if (highlightTheme) {
    highlightTheme.href = "/css/github-dark.min.css";
  }
}

// Toggle light mode
lightModeToggle.addEventListener("click", () => {
  body.classList.toggle("light-mode");

  if (body.classList.contains("light-mode")) {
    lightModeToggle.textContent = "🌙";
    localStorage.setItem("theme", "light");
    if (highlightTheme) {
      highlightTheme.href = "/css/github.min.css";
    }
  } else {
    lightModeToggle.textContent = "☀️";
    localStorage.setItem("theme", "dark");
    if (highlightTheme) {
      highlightTheme.href = "/css/github-dark.min.css";
    }
  }
});

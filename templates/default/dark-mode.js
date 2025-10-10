const lightModeToggle = document.getElementById("lightModeToggle");
const body = document.body;

// Load saved preference or default to dark
const savedTheme = localStorage.getItem("theme");
if (savedTheme === "light") {
  body.classList.add("light-mode");
  lightModeToggle.textContent = "🌙";
} else {
  body.classList.remove("light-mode");
  lightModeToggle.textContent = "☀️";
}

// Toggle light mode
lightModeToggle.addEventListener("click", () => {
  body.classList.toggle("light-mode");

  if (body.classList.contains("light-mode")) {
    lightModeToggle.textContent = "🌙";
    localStorage.setItem("theme", "light");
  } else {
    lightModeToggle.textContent = "☀️";
    localStorage.setItem("theme", "dark");
  }
});

// compute arity of books and toggle table layout when comments of a books are
// expanded/retracted
const booksContainer = document.getElementById("books-container");
const books = document.getElementsByClassName("book");
{
    let even = true;
    for (const book of books) {
        book.classList.add("a" + (even ? 1 : 0));
        let collapser = null;
        for (const child of book.children) {
            if (child.classList.contains("collapser")) {
                collapser = child;
                break;
            }
        }
        if (collapser == null) continue;
        collapser.addEventListener("click", function() {
            booksContainer.style["grid-template-columns"] = book.classList.contains("a1" ? "5% 5%" : "5% 95%");
        });
        even = !even;
    }
}



function hamburgerToggle() {
    const nav = document.getElementById("navlinks");
    if (nav.style.display === "grid") {
        nav.style.display = "none";
    } else {
        nav.style.display = "grid";
    }
}

function getSiblingByClass(e, className) {
    const siblings = e.parentElement.children;
    for (const sibling of siblings) {
        if (sibling.classList.contains(className)) {
            return sibling;
        }
    }
}

const readMoreThreshold = 225;
const folds = document.getElementsByClassName("fold");
const MORE = "...&nbsp;Mehr&nbsp;anzeigen";
const LESS = "&nbsp;(Weniger&nbsp;anzeigen)";
for (let i = 0; i < folds.length; i++) {
    const e = folds[i];
    if (e.innerHTML.length >= readMoreThreshold) {
        e.innerHTML = e.innerHTML.substring(0, readMoreThreshold)
            + '<span class="more">' + MORE + '</span><span class="more-content" style="display:none;">'
            + e.innerHTML.substring(readMoreThreshold)
            + '</span><span class="less" style="display:none;">' + LESS + '</span>';
    }
}

const more = document.getElementsByClassName("more");
for (let i = 0; i < more.length; i++) {
    const e = more[i];
    e.addEventListener("click", function() {
        const content = getSiblingByClass(this, "more-content");
        content.style.display = "inline";
        getSiblingByClass(this, "less").style.display = "inline";
        this.style.display = "none";
    });
}

const less = document.getElementsByClassName("less");
for (let i = 0; i < less.length; i++) {
    const e = less[i];
    e.addEventListener("click", function() {
        const content = getSiblingByClass(this, "more-content");
        content.style.display = "none";
        getSiblingByClass(this, "more").style.display = "inline";
        this.style.display = "none";
    });
}

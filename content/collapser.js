var collapserClassName = collapserClassName || "collapser";
var collapserInitShowClassName = collapserInitShowClassName || "collapser-init-show";
var collapserActivePrefix = collapserActivePrefix || "&blacktriangledown; ";
var collapserInactivePrefix = collapserInactivePrefix || "&blacktriangleright; ";
var collapserDisplayShow = collapserDisplayShow || "block";
var collapserDisplayHide = collapserDisplayHide || "none";

const colls = document.getElementsByClassName(collapserClassName);

for (let i = 0; i < colls.length; i++) {
    const coll = colls[i];
    if (coll.classList.contains(collapserInitShowClassName)) {
        coll.innerHTML = collapserActivePrefix + coll.innerHTML;
    } else {
        coll.innerHTML = collapserInactivePrefix + coll.innerHTML;
        coll.nextElementSibling.style.display = "none";
    }
    coll.addEventListener("click", function() {
        const content = this.nextElementSibling;
        if (content.style.display === collapserDisplayShow) {
            this.innerHTML = collapserInactivePrefix + this.innerHTML.substring(1);
            content.style.display = collapserDisplayHide;
        } else {
            this.innerHTML = collapserActivePrefix + this.innerHTML.substring(1);
            content.style.display = collapserDisplayShow;
        }
    });
}

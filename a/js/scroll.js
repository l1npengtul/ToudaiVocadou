const json = fetch("/works_list.json").then(resp => resp.json())

function random_sort(arr) {
  	return arr
    	.map((val) => ({ val, sort: Math.random() }))
    	.sort((a, b) => a.sort - b.sort)
    	.map(({ val }) => val);
}

function create_element(work) {
    let root = document.createElement("div")
    root.className = "card"
    root.innerHTML = work.embed_html
    let link = document.createElement("a");
    link.href = work.on_site_link;
    let title = document.createElement("h6");
    title.innerHTML = work.title;
    link.appendChild(title)
    let author_link = document.createElement("a")
    author_link.href = work.author_link
    let author_elem = document.createElement("p");
    author_elem.innerHTML = work.author_displayname
    author_elem.style = "font-size: 1rem;"
    author_link.appendChild(author_elem)
    root.appendChild(link)
    root.appendChild(author_link)
    return root
}

document.addEventListener('DOMContentLoaded', async function() {
    let works = random_sort(await json);
    
    let a_scroll = document.getElementById("visible-slider-group");
    let b_scroll = document.getElementById("hidden-slider-group");
    works.forEach(element => {
        a_scroll.appendChild(create_element(element))
        b_scroll.appendChild(create_element(element))
    });
})

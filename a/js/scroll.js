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
    return root
}

document.addEventListener('DOMContentLoaded', async function() {
    let works = random_sort(await json).slice(0, 8);
    
    let a_scroll = document.getElementById("visible-slider-group");
    let b_scroll = document.getElementById("hidden-slider-group");
    works.forEach(element => {
        a_scroll.appendChild(create_element(element))
        b_scroll.appendChild(create_element(element))
    });
})

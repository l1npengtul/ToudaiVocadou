const json = fetch("/works_list.json").then(resp => resp.json())

function titleElement(title, link) {
    document.getElementById("featured-work-title").innerHTML = title
    document.getElementById("featured-work-link").href = link
}

function creatorElement(author, author_link) {
    let creator = document.getElementById("featured-work-creator")
    creator.innerHTML = author
    creator.href = `/members/{author_link}.html`
}

function descriptionElement(desc) {
    document.getElementById("featured-work-description").innerHTML = desc
}

function embedElement(embed) {
    document.getElementById("embed").innerHTML = embed
}

async function pickRandomRenderHtml() {
    let workslist = await json;
    let picked = workslist[Math.floor(Math.random() * workslist.length)]

    if (picked.description !== null) {
        descriptionElement(picked.description);
    }

    titleElement(picked.title, picked.on_site_link)
    creatorElement(picked.author_displayname, picked.author_link)
    embedElement(picked.embed_html)
}

document.addEventListener('DOMContentLoaded', function() {
    // スムーズスクロール機能
    const smoothScrollLinks = document.querySelectorAll('a[href^="#"]');
    
    for (const link of smoothScrollLinks) {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            
            const targetId = this.getAttribute('href');
            const targetElement = document.querySelector(targetId);
            
            if (targetElement) {
                const offsetTop = targetElement.getBoundingClientRect().top + window.pageYOffset;
                
                window.scrollTo({
                    top: offsetTop,
                    behavior: 'smooth'
                });
            }
        });
    }
    
    // ナビゲーションのアクティブクラス制御
    const sections = document.querySelectorAll('section[id]');
    
    window.addEventListener('scroll', function() {
        const scrollPosition = window.scrollY + 200;
        
        for (const section of sections) {
            const sectionTop = section.offsetTop;
            const sectionHeight = section.offsetHeight;
            const sectionId = section.getAttribute('id');
            
            if (scrollPosition >= sectionTop && scrollPosition < sectionTop + sectionHeight) {
                const navLinks = document.querySelectorAll('nav ul li a');
                
                for (const link of navLinks) {
                    if (link.getAttribute('href').includes(sectionId)) {
                        document.querySelectorAll('nav ul li a').forEach(el => el.classList.remove('active'));
                        link.classList.add('active');
                    }
                }
            }
        } });

        document.getElementById("reload").addEventListener("click", pickRandomRenderHtml)

        pickRandomRenderHtml()
    });

    // プレースホルダー画像のテキスト表示
    const imgPlaceholders = document.querySelectorAll('.img-placeholder');
    
    for (const placeholder of imgPlaceholders) {
        if (!placeholder.textContent) {
            placeholder.textContent = '画像準備中';
        }
    }
;

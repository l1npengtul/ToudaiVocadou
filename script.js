document.addEventListener('DOMContentLoaded', function() {
    const catchphrase = document.querySelector('.catchphrase');
    const backgroundImage = document.querySelector('.background-image');
    
    catchphrase.classList.add('slide-in');
    backgroundImage.classList.add('blur');

    const members = document.querySelectorAll('.member');
    const leftArrow = document.querySelector('.arrow-left');
    const rightArrow = document.querySelector('.arrow-right');
    let currentIndex = 0;

    function updateMembers() {
        members.forEach((member, index) => {
            if (index >= currentIndex && index < currentIndex + 3) {
                member.style.display = 'block';
            } else {
                member.style.display = 'none';
            }
        });
        if (currentIndex === 0) {
            leftArrow.style.opacity = 0;
        }
        else if (currentIndex === 1) {
            leftArrow.style.opacity = 0.5;
        }
        if (currentIndex === members.length - 3) {
            rightArrow.style.opacity = 0;
        }
        else if (currentIndex === members.length - 4) {
            rightArrow.style.opacity = 0.5;
        }
    }

    leftArrow.addEventListener('click', () => {
        if (currentIndex > 0) {
            currentIndex--;
            updateMembers();
        }
    });

    rightArrow.addEventListener('click', () => {
        if (currentIndex < members.length - 3) {
            currentIndex++;
            updateMembers();
        }
    });

    updateMembers();
});

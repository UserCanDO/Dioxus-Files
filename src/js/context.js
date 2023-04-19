document.addEventListener("click", (e) => {
    // ? close the menu if the user clicks outside of it
    document.getElementsByClassName("context-visible").forEach((contextMenu)=>{
        if (e.target.offsetParent != contextMenu) {
            contextMenu.classList.remove("context-visible");
        }
    });
});

/*const normalizePozition = (contextMenu, mouseX, mouseY) => {
    // ? compute what is the mouse position relative to the container element (scope)
    let { left: scopeOffsetX, top: scopeOffsetY, } = scope.getBoundingClientRect();

    scopeOffsetX = scopeOffsetX < 0 ? 0 : scopeOffsetX;
    scopeOffsetY = scopeOffsetY < 0 ? 0 : scopeOffsetY;

    const scopeX = mouseX - scopeOffsetX;
    const scopeY = mouseY - scopeOffsetY;

    // ? check if the element will go out of bounds
    const outOfBoundsOnX =  scopeX + contextMenu.clientWidth > scope.clientWidth;
    const outOfBoundsOnY = scopeY + contextMenu.clientHeight > scope.clientHeight;

    let normalizedX = mouseX;
    let normalizedY = mouseY;

    // ? normalize on X
    if (outOfBoundsOnX) {
        normalizedX = scopeOffsetX + scope.clientWidth - contextMenu.clientWidth;
    }

    // ? normalize on Y
    if (outOfBoundsOnY) {
        normalizedY = scopeOffsetY + scope.clientHeight - contextMenu.clientHeight;
    }

    return { normalizedX, normalizedY };
};

document.getElementsByClassName("contextmenu").forEach(contextMenu =>{
    contextMenu.addEventListener("contextmenu", (event) => {
        event.preventDefault();
        
        const { clientX: mouseX, clientY: mouseY } = event;
        const { normalizedX, normalizedY } = normalizePozition(contextMenu, mouseX, mouseY);
    
        contextMenu.classList.remove("context-visible");
    
        contextMenu.style.top = `${normalizedY}px`;
        contextMenu.style.left = `${normalizedX}px`;
    
        setTimeout(() => {
            contextMenu.classList.add("context-visible");
        });
    });
    
    
});*/
/**
 * UniversalPath.org - Main JavaScript
 */

document.addEventListener('DOMContentLoaded', function() {
    // Mobile navigation toggle
    const mobileMenuButton = document.querySelector('.mobile-menu-button');
    const mobileMenu = document.querySelector('.mobile-menu');
    
    if (mobileMenuButton && mobileMenu) {
        mobileMenuButton.addEventListener('click', function() {
            mobileMenu.classList.toggle('hidden');
        });
    }
    
    // Handle article content links
    const articleContent = document.querySelector('.article-content');
    if (articleContent) {
        articleContent.addEventListener('click', function(event) {
            const target = event.target;
            if (target.tagName === 'A' && target.getAttribute('href').startsWith('/')) {
                // Internal link, let it happen normally
            } else if (target.tagName === 'A') {
                // External link, open in new tab
                event.preventDefault();
                window.open(target.getAttribute('href'), '_blank');
            }
        });
    }
    
    // Smooth scroll to anchors
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function(e) {
            e.preventDefault();
            
            const targetId = this.getAttribute('href');
            if (targetId === '#') return;
            
            const targetElement = document.querySelector(targetId);
            if (targetElement) {
                targetElement.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });
    
    // Handle search form submission
    const searchForm = document.querySelector('form[action="/search"]');
    if (searchForm) {
        searchForm.addEventListener('submit', function(event) {
            const searchInput = this.querySelector('input[name="q"]');
            if (!searchInput || searchInput.value.trim() === '') {
                event.preventDefault();
            }
        });
    }
    
    // Add table classes to any tables in article content
    const articleTables = document.querySelectorAll('.article-content table');
    articleTables.forEach(table => {
        table.classList.add('w-full', 'border-collapse', 'border', 'border-gray-300', 'my-4');
        
        const headerCells = table.querySelectorAll('th');
        headerCells.forEach(cell => {
            cell.classList.add('bg-gray-100', 'border', 'border-gray-300', 'p-2', 'text-left');
        });
        
        const cells = table.querySelectorAll('td');
        cells.forEach(cell => {
            cell.classList.add('border', 'border-gray-300', 'p-2');
        });
    });
    
    // Highlight current menu item
    const currentPath = window.location.pathname;
    const navLinks = document.querySelectorAll('nav a');
    
    navLinks.forEach(link => {
        if (link.getAttribute('href') === currentPath) {
            link.classList.add('text-yellow-300');
        }
    });
    
    // Lazy load images (simple implementation)
    if ('IntersectionObserver' in window) {
        const lazyImages = document.querySelectorAll('img[data-src]');
        
        const imageObserver = new IntersectionObserver((entries, observer) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    const img = entry.target;
                    img.src = img.dataset.src;
                    img.removeAttribute('data-src');
                    imageObserver.unobserve(img);
                }
            });
        });
        
        lazyImages.forEach(img => {
            imageObserver.observe(img);
        });
    } else {
        // Fallback for browsers that don't support IntersectionObserver
        const lazyImages = document.querySelectorAll('img[data-src]');
        lazyImages.forEach(img => {
            img.src = img.dataset.src;
            img.removeAttribute('data-src');
        });
    }
});

// Utility functions
function debounce(func, wait) {
    let timeout;
    return function(...args) {
        clearTimeout(timeout);
        timeout = setTimeout(() => func.apply(this, args), wait);
    };
}
// ==UserScript==
// @name         F95 Random Bookmark Selector
// @namespace    https://github.com/ZimCodes/bonus/tree/main/user-scripts
// @version      2024-02-25
// @description  randomly picks a page & bookmark number from your f95 bookmark page. Just press the 'Random' button. 
// @author       ZimCodes
// @match        https://f95zone.to/account/bookmarks*
// @icon         https://www.google.com/s2/favicons?sz=64&domain=tampermonkey.net
// @grant        none
// ==/UserScript==

(() => {
    'use strict';

    var randBtn = document.createElement('button');
    //Prepare button attributes
    randBtn.setAttribute("name","random");
    randBtn.setAttribute("type","button");
	randBtn.id = "randbtn";
	//F95zone style classes
	randBtn.classList.add("pageNav-jump","pageNav-jump--next");
	
    //Prepare button text
	var randTxt = document.createTextNode('Random');
	randBtn.appendChild(randTxt);
	
	//button CSS styles
	var btnCSS = (`
		#randbtn{
			color:yellow;
			border: 1px solid #343638;
			
		}
		#randbtn:hover{
			cursor: pointer;
			opacity: 0.7;
		}
		#randbtn:active{
			background-color: #ec5555;
		}
	`);
	
	//Apply button styles
	function applyCSS(css) {
	  const style = document.createElement('style');
	  style.appendChild(document.createTextNode(css));
	  document.head.appendChild(style);
	}
	applyCSS(btnCSS);
	
	//Add button to F95
	document.querySelector("nav > div.pageNav").appendChild(randBtn);
	
	function pickRandomBookmark(){
		var pagination = document.querySelector("ul.pageNav-main");//Select pagination
		var lastPageNum = pagination.children[pagination.children.length-1].children[0].textContent;//Get the last page number of the pagination
		
		
		function getRandIntInc(min, max) {
			//Inclusive random number generator
			return Math.floor(Math.random() * (max - min + 1) ) + min;
		} 
		
		var randPageChoiceNum = getRandIntInc(1,Number(lastPageNum));//Pick a random page number
		var randBookmarkChoiceNum = getRandIntInc(1,20);// Pick a random bookmark position (there can only be a max of 20 bookmarks per pages)
		alert(`Page #${randPageChoiceNum} \nBookmark #${randBookmarkChoiceNum}`); //Show an alert 
	}
	randBtn.addEventListener("click", pickRandomBookmark);
})();
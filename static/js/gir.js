function gir() {
    let nums = document.getElementById('gir').className; 
    if(nums == 'gir_1') {document.getElementById('gir').className='gir_2';}  
    if(nums == 'gir_2') {document.getElementById('gir').className='gir_3';}  
    if(nums == 'gir_3') {document.getElementById('gir').className='gir_1';} 
}  
setInterval(gir, 500); 
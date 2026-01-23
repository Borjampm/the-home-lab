<script lang="ts">                                                                  
  import { onMount, onDestroy } from 'svelte';                                      
  import { invoke, Channel } from '@tauri-apps/api/core';                           
  import { Terminal } from '@xterm/xterm';                                          
  import { FitAddon } from '@xterm/addon-fit';                                      
  import { WebglAddon } from '@xterm/addon-webgl';                                  
  import '@xterm/xterm/css/xterm.css';                                              
                                                                                    
  let terminalEl: HTMLDivElement;                                                   
  let term: Terminal;                                                               
  let fitAddon: FitAddon;                                                           
                                                                                    
  type PtyEvent =                                                                   
    | { event: 'data'; data: string }                                               
    | { event: 'exit'; data: number };                                              
                                                                                    
  onMount(async () => {                                                             
    term = new Terminal({                                                           
      cursorBlink: true,                                                            
      fontSize: 14,                                                                 
      fontFamily: 'Menlo, Monaco, "Courier New", monospace',                        
      theme: {                                                                      
        background: '#1e1e2e',                                                      
        foreground: '#cdd6f4',                                                      
        cursor: '#f5e0dc',                                                          
      },                                                                            
    });                                                                             
                                                                                    
    fitAddon = new FitAddon();                                                      
    term.loadAddon(fitAddon);                                                       
    term.open(terminalEl);                                                          
                                                                                    
    try {                                                                           
      const webgl = new WebglAddon();                                               
      webgl.onContextLoss(() => webgl.dispose());                                   
      term.loadAddon(webgl);                                                        
    } catch (e) {                                                                   
      console.warn('WebGL unavailable, using DOM renderer');                        
    }                                                                               
                                                                                    
    fitAddon.fit();                                                                 
                                                                                    
    const onData = new Channel<PtyEvent>();                                         
    onData.onmessage = (msg: PtyEvent) => {                                         
      if (msg.event === 'data') term.write(msg.data);                               
      if (msg.event === 'exit') term.write('\r\n[Process exited]\r\n');             
    };                                                                              
                                                                                    
    await invoke('spawn_shell', { onData, rows: term.rows, cols: term.cols });      
                                                                                    
    term.onData((data) => invoke('write_to_pty', { data }));                        
    term.onResize(({ rows, cols }) => invoke('resize_pty', { rows, cols }));        
                                                                                    
    window.addEventListener('resize', handleResize);                                
  });                                                                               
                                                                                    
  function handleResize() {                                                         
    fitAddon?.fit();                                                                
  }                                                                                 
                                                                                    
  onDestroy(() => {                                                                 
    window.removeEventListener('resize', handleResize);                             
    term?.dispose();                                                                
  });                                                                               
</script>                                                                           
                                                                                    
<main>                                                                              
  <div bind:this={terminalEl} class="terminal-container"></div>                     
</main>                                                                             
                                                                                    
<style>                                                                             
  :global(body) {                                                                   
    margin: 0;                                                                      
    padding: 0;                                                                     
    overflow: hidden;                                                               
    background: #1e1e2e;                                                            
  }                                                                                 
  main {                                                                            
    width: 100vw;                                                                   
    height: 100vh;                                                                  
    display: flex;                                                                  
  }                                                                                 
  .terminal-container {                                                             
    flex: 1;                                                                        
    padding: 4px;                                                                   
  }                                                                                 
  .terminal-container :global(.xterm) {                                             
    height: 100%;                                                                   
  }                                                                                 
</style>   
(function() {var implementors = {};
implementors["particle_behaviour"] = [{"text":"impl NetworkBehaviour for ParticleBehaviour","synthetic":false,"types":[]}];
implementors["particle_server"] = [{"text":"impl NetworkBehaviour for ServerBehaviour <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Bootstrapper: NetworkBehaviour,<br>&nbsp;&nbsp;&nbsp;&nbsp;Self: NetworkBehaviourEventProcess&lt;&lt;Bootstrapper as NetworkBehaviour&gt;::OutEvent&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Identify: NetworkBehaviour,<br>&nbsp;&nbsp;&nbsp;&nbsp;Self: NetworkBehaviourEventProcess&lt;&lt;Identify as NetworkBehaviour&gt;::OutEvent&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;Ping: NetworkBehaviour,<br>&nbsp;&nbsp;&nbsp;&nbsp;Self: NetworkBehaviourEventProcess&lt;&lt;Ping as NetworkBehaviour&gt;::OutEvent&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;ParticleBehaviour: NetworkBehaviour,<br>&nbsp;&nbsp;&nbsp;&nbsp;Self: NetworkBehaviourEventProcess&lt;&lt;ParticleBehaviour as NetworkBehaviour&gt;::OutEvent&gt;,&nbsp;</span>","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()
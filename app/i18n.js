(() => {
  'use strict';

  const STORAGE_KEY = 'heartlight.language';
  const supported = ['en', 'es'];
  const es = new Map([
    ['Inclusive learning, regulation, and communication support — local-first.', 'Apoyo inclusivo para el aprendizaje, la regulación y la comunicación — primero en el dispositivo.'],
    ['No cloud. No account. Session data stays on this device.', 'Sin nube. Sin cuenta. Los datos de la sesión permanecen en este dispositivo.'],
    ['App sections', 'Secciones de la aplicación'],
    ['Child', 'Niño/a'],
    ['Teacher', 'Docente'],
    ['Color Sensor', 'Sensor de color'],
    ['Study', 'Estudio'],
    ['Settings', 'Ajustes'],
    ['What do you need?', '¿Qué necesitas?'],
    ['Pick as many as you want. “I don’t know” is a complete answer.', 'Elige todas las opciones que quieras. “No lo sé” es una respuesta completa.'],
    ['Show gentle options', 'Mostrar opciones suaves'],
    ['Clear', 'Borrar'],
    ['My body right now', 'Mi cuerpo ahora'],
    ['Energy / movement need', 'Energía / necesidad de movimiento'],
    ['Sensory load', 'Carga sensorial'],
    ['Focus feels possible', 'Siento que puedo concentrarme'],
    ['I know what would help', 'Sé qué podría ayudarme'],
    ['Support options', 'Opciones de apoyo'],
    ['Choose needs, then press “Show gentle options.”', 'Elige necesidades y luego pulsa “Mostrar opciones suaves”.'],
    ['Visual timer', 'Temporizador visual'],
    ['Start', 'Iniciar'],
    ['Pause', 'Pausar'],
    ['Reset', 'Reiniciar'],
    ['A timer is an option, not a demand. Adults should not use it as a punishment.', 'El temporizador es una opción, no una exigencia. Los adultos no deben usarlo como castigo.'],
    ['Teacher / aide observation', 'Observación del docente / asistente'],
    ['Describe what happened without assigning motive. Use a student code, not a full name.', 'Describe lo ocurrido sin asignar motivos. Usa un código del estudiante, no el nombre completo.'],
    ['Student code', 'Código del estudiante'],
    ['e.g., Room3-A', 'p. ej., Aula3-A'],
    ['Context / antecedent', 'Contexto / antecedente'],
    ['What was happening immediately before?', '¿Qué estaba ocurriendo inmediatamente antes?'],
    ['Observable behavior / communication', 'Conducta / comunicación observable'],
    ['What could a camera have seen or microphone heard?', '¿Qué podría haber visto una cámara u oído un micrófono?'],
    ['Support offered', 'Apoyo ofrecido'],
    ['Choice, quieter area, movement, visual schedule, break...', 'Elección, zona más tranquila, movimiento, horario visual, descanso...'],
    ['What happened next', 'Qué ocurrió después'],
    ['What changed? What did the student communicate?', '¿Qué cambió? ¿Qué comunicó el estudiante?'],
    ['Save locally', 'Guardar localmente'],
    ['Export CSV', 'Exportar CSV'],
    ['Erase local logs', 'Borrar registros locales'],
    ['CST 12D support-state', 'Estado de apoyo CST 12D'],
    ['A transparent classroom projection. These are support variables, not diagnoses or personality scores.', 'Una proyección transparente para el aula. Son variables de apoyo, no diagnósticos ni puntuaciones de personalidad.'],
    ['Pattern review', 'Revisión de patrones'],
    ['Saved observations are summarized locally. The app does not rank children or predict misconduct.', 'Las observaciones guardadas se resumen localmente. La app no clasifica a los niños ni predice mala conducta.'],
    ['Human decision boundary', 'Límite de decisión humana'],
    ['Never automate:', 'Nunca automatizar:'],
    ['diagnosis, discipline, restraint, seclusion, medication advice, eligibility, IEP decisions, risk labels, or denial of access. Use HEARTLIGHT to surface possible supports for a qualified human team to consider.', 'diagnóstico, disciplina, restricción, aislamiento, consejos de medicación, elegibilidad, decisiones del IEP, etiquetas de riesgo o negación de acceso. Usa HEARTLIGHT para mostrar posibles apoyos que un equipo humano cualificado pueda considerar.'],
    ['Ambient color / light sensor', 'Sensor de color / luz ambiental'],
    ['Optional camera sampling estimates the dominant color and brightness of the environment. Frames are processed in memory and are not intentionally saved or uploaded.', 'El muestreo opcional de la cámara estima el color dominante y el brillo del entorno. Los fotogramas se procesan en memoria y HEARTLIGHT no los guarda ni los sube intencionalmente.'],
    ['Get appropriate adult/school permission before using a camera around children. Point the camera at the room/light source, not at a student.', 'Obtén el permiso correspondiente del adulto o de la escuela antes de usar una cámara cerca de niños. Apunta la cámara a la habitación o fuente de luz, no al estudiante.'],
    ['Start sensor', 'Iniciar sensor'],
    ['Stop', 'Detener'],
    ['Current reading', 'Lectura actual'],
    ['Brightness: —', 'Brillo: —'],
    ['No reading yet.', 'Todavía no hay lectura.'],
    ['Optional observed pulse (manual, not measured by app)', 'Pulso observado opcional (manual; la app no lo mide)'],
    ['Optional', 'Opcional'],
    ['Pulse is informational only and is not used for medical interpretation. Do not use this app to evaluate illness or emergencies.', 'El pulso es solo informativo y no se usa para interpretación médica. No uses esta app para evaluar enfermedades o emergencias.'],
    ['Color modulation rule', 'Regla de modulación del color'],
    ['The app never assumes one color is “calming” for everyone. It combines the room reading with the child’s selected needs and accessibility settings. If brightness is high and “too bright” is selected, it can suggest reducing glare or offering a lower-light option. The child’s preference wins.', 'La app nunca supone que un color sea “calmante” para todos. Combina la lectura de la sala con las necesidades seleccionadas por el niño y los ajustes de accesibilidad. Si el brillo es alto y se selecciona “demasiado brillante”, puede sugerir reducir el deslumbramiento u ofrecer una opción con menos luz. La preferencia del niño tiene prioridad.'],
    ['Study arc', 'Ruta de estudio'],
    ['Observe without labeling.', 'Observar sin etiquetar.'],
    ['Ask what the learner is communicating.', 'Preguntar qué está comunicando el estudiante.'],
    ['Separate sensory load, task demand, communication load, and regulation need.', 'Separar la carga sensorial, la demanda de la tarea, la carga de comunicación y la necesidad de regulación.'],
    ['Offer choices and access supports.', 'Ofrecer elecciones y apoyos de acceso.'],
    ['Record only what is necessary.', 'Registrar solo lo necesario.'],
    ['Review patterns with the student and team.', 'Revisar patrones con el estudiante y el equipo.'],
    ['Test one small change at a time.', 'Probar un pequeño cambio a la vez.'],
    ['Keep what helps; discard what does not.', 'Mantener lo que ayuda; descartar lo que no.'],
    ['COSMOS learning loop', 'Ciclo de aprendizaje COSMOS'],
    ['Perceive → compress → expand → validate → express → store.', 'Percibir → comprimir → expandir → validar → expresar → guardar.'],
    ['In HEARTLIGHT: observe signals → summarize the situation → generate multiple support hypotheses → check against the learner’s communication and context → offer choices → keep only useful, consented notes.', 'En HEARTLIGHT: observar señales → resumir la situación → generar varias hipótesis de apoyo → comprobarlas con la comunicación y el contexto del estudiante → ofrecer opciones → conservar solo notas útiles y consentidas.'],
    ['Stimming principle', 'Principio sobre el stimming'],
    ['Repetitive movement or sound may support regulation, attention, expression, or joy. The default response is not “stop the stim.” First ask whether it is safe, whether it is harming the learner or another person, and whether the environment can be changed.', 'El movimiento o sonido repetitivo puede apoyar la regulación, la atención, la expresión o la alegría. La respuesta predeterminada no es “detener el stim”. Primero pregunta si es seguro, si está causando daño al estudiante o a otra persona y si se puede cambiar el entorno.'],
    ['Open the manuals', 'Abrir los manuales'],
    ['The repository includes full teacher, study, stimming, clinician-adjunct, privacy, architecture, and validation guides.', 'El repositorio incluye guías completas para docentes, estudio, stimming, uso clínico complementario, privacidad, arquitectura y validación.'],
    ['Accessibility', 'Accesibilidad'],
    ['High contrast', 'Alto contraste'],
    ['Reduce motion', 'Reducir movimiento'],
    ['Text size', 'Tamaño del texto'],
    ['Privacy', 'Privacidad'],
    ['Remember accessibility settings on this device', 'Recordar los ajustes de accesibilidad en este dispositivo'],
    ['Student check-ins are session-only. Teacher logs are stored only when “Save locally” is pressed. Camera frames are not stored.', 'Las selecciones del estudiante duran solo la sesión. Los registros del docente se guardan únicamente al pulsar “Guardar localmente”. Los fotogramas de la cámara no se almacenan.'],
    ['Erase all HEARTLIGHT data', 'Borrar todos los datos de HEARTLIGHT'],
    ['About the engine', 'Acerca del motor'],
    ['HEARTLIGHT adapts the COSMOS/CST local-first loop into an educational support tool. “Memory” here means bounded, reviewable records — not permanent child profiling. All recommendations are deterministic and inspectable in the source.', 'HEARTLIGHT adapta el ciclo local COSMOS/CST a una herramienta de apoyo educativo. Aquí “memoria” significa registros limitados y revisables, no perfiles permanentes de menores. Todas las recomendaciones son deterministas y pueden inspeccionarse en el código fuente.'],
    ['HEARTLIGHT is an educational support and accessibility tool, not a medical device, diagnostic system, psychotherapy replacement, crisis service, or substitute for qualified special-education/clinical judgment.', 'HEARTLIGHT es una herramienta de apoyo educativo y accesibilidad, no un dispositivo médico, sistema de diagnóstico, sustituto de psicoterapia, servicio de crisis ni sustituto del juicio profesional cualificado de educación especial o clínica.'],
    ['🤫 Quiet', '🤫 Silencio'],
    ['🌀 Move', '🌀 Moverme'],
    ['🕯️ Less light', '🕯️ Menos luz'],
    ['↔️ More space', '↔️ Más espacio'],
    ['🧩 Help me', '🧩 Ayúdame'],
    ['🌿 Break', '🌿 Descanso'],
    ['🧸 Pressure', '🧸 Presión'],
    ['🎧 Sound choice', '🎧 Elegir sonido'],
    ['🗓️ Tell me what happens next', '🗓️ Dime qué sigue'],
    ['💬 Another way to communicate', '💬 Otra forma de comunicarme'],
    ['❔ I don’t know', '❔ No lo sé'],
    ['💚 Stay with me', '💚 Quédate conmigo'],
    ['Offer a quieter place, hearing protection if already approved, or reduce competing sound.', 'Ofrece un lugar más tranquilo, protección auditiva si ya está aprobada o reduce los sonidos que compiten.'],
    ['Offer safe movement: walk, stretch, rocking seat, wall push, or movement break.', 'Ofrece movimiento seguro: caminar, estirarse, asiento mecedor, empujar la pared o un descanso de movimiento.'],
    ['Reduce glare, dim lights where safe, change seat, or offer a visor/hat if permitted.', 'Reduce el deslumbramiento, baja las luces cuando sea seguro, cambia de asiento u ofrece una visera o gorra si está permitida.'],
    ['Offer more physical space without isolating the learner as punishment.', 'Ofrece más espacio físico sin aislar al estudiante como castigo.'],
    ['Break the task into one visible next step and offer a choice of how to begin.', 'Divide la tarea en un siguiente paso visible y ofrece una opción sobre cómo empezar.'],
    ['Offer a predictable, time-flexible break and a clear path back when ready.', 'Ofrece un descanso predecible y flexible, con un camino claro para volver cuando esté listo o lista.'],
    ['Offer only familiar, consented proprioceptive options such as pushing a wall or carrying a light classroom item. Never impose touch.', 'Ofrece solo opciones propioceptivas conocidas y consentidas, como empujar una pared o llevar un objeto ligero del aula. Nunca impongas contacto físico.'],
    ['Offer a preferred sound level or approved headphones; do not force silence.', 'Ofrece el nivel de sonido preferido o auriculares aprobados; no obligues al silencio.'],
    ['Show a first→then card, visual schedule, transition warning, or countdown.', 'Muestra una tarjeta primero→después, horario visual, aviso de transición o cuenta regresiva.'],
    ['Offer AAC, typing, pointing, drawing, gesture, or yes/no choices. Speech is not the only valid communication.', 'Ofrece CAA/AAC, escritura, señalar, dibujar, gestos u opciones de sí/no. El habla no es la única comunicación válida.'],
    ['Reduce demands briefly and offer two simple options. “I don’t know” may mean the learner needs processing time.', 'Reduce brevemente las demandas y ofrece dos opciones sencillas. “No lo sé” puede significar que necesita tiempo para procesar.'],
    ['Stay nearby, use fewer words, and let the learner control distance when possible.', 'Quédate cerca, usa menos palabras y permite que el estudiante controle la distancia cuando sea posible.'],
    ['No need selected. You can still take a quiet moment.', 'No se seleccionó ninguna necesidad. Aun así puedes tomarte un momento tranquilo.'],
    ['The room sensor also reads bright. Consider a lower-glare option if the learner agrees.', 'El sensor de la sala también detecta mucho brillo. Considera una opción con menos deslumbramiento si el estudiante está de acuerdo.'],
    ['Add at least a context or observable behavior note.', 'Añade al menos una nota de contexto o de conducta observable.'],
    ['Saved on this device only.', 'Guardado solo en este dispositivo.'],
    ['Erase all locally saved teacher logs?', '¿Borrar todos los registros docentes guardados localmente?'],
    ['No saved observations yet.', 'Todavía no hay observaciones guardadas.'],
    ['Most-recorded supports (frequency only; not effectiveness):', 'Apoyos registrados con mayor frecuencia (solo frecuencia; no efectividad):'],
    ['Review actual outcomes with the learner/team before deciding whether a support helps.', 'Revisa los resultados reales con el estudiante y el equipo antes de decidir si un apoyo ayuda.'],
    ['Camera unavailable or permission was declined. The rest of HEARTLIGHT works without it.', 'La cámara no está disponible o se rechazó el permiso. El resto de HEARTLIGHT funciona sin ella.'],
    ['Environment appears bright. Offer glare reduction only if the learner prefers it.', 'El entorno parece brillante. Ofrece reducir el deslumbramiento solo si el estudiante lo prefiere.'],
    ['Environment appears dim. Check that visual materials remain readable and safe.', 'El entorno parece tenue. Comprueba que los materiales visuales sigan siendo legibles y seguros.'],
    ['Light level appears moderate. Learner preference remains the deciding signal.', 'El nivel de luz parece moderado. La preferencia del estudiante sigue siendo la señal decisiva.'],
    ['Erase all HEARTLIGHT settings and teacher logs from this device?', '¿Borrar todos los ajustes y registros docentes de HEARTLIGHT de este dispositivo?'],
    ['sensory load', 'carga sensorial'],
    ['visual load', 'carga visual'],
    ['movement need', 'necesidad de movimiento'],
    ['focus access', 'acceso a la concentración'],
    ['transition need', 'necesidad de transición'],
    ['communication load', 'carga de comunicación'],
    ['social space need', 'necesidad de espacio social'],
    ['body comfort', 'comodidad corporal'],
    ['predictability need', 'necesidad de previsibilidad'],
    ['recovery need', 'necesidad de recuperación'],
    ['engagement access', 'acceso a la participación'],
    ['regulation confidence', 'confianza en la regulación']
  ]);

  let language = 'en';
  let applying = false;
  const originalText = new WeakMap();
  const originalPlaceholder = new WeakMap();
  const originalAria = new WeakMap();

  function normalizedLanguage(value) {
    value = String(value || '').toLowerCase();
    return supported.includes(value) ? value : (value.startsWith('es') ? 'es' : 'en');
  }

  function translateString(value) {
    if (language !== 'es' || typeof value !== 'string') return value;
    const exact = es.get(value);
    if (exact) return exact;
    const trimmed = value.trim();
    const translatedTrimmed = es.get(trimmed);
    if (translatedTrimmed) return value.replace(trimmed, translatedTrimmed);
    const brightness = value.match(/^Brightness estimate: (\d+)\/255$/);
    if (brightness) return `Estimación de brillo: ${brightness[1]}/255`;
    const observations = value.match(/^(\d+) local observation\(s\)\.$/);
    if (observations) return `${observations[1]} observación(es) local(es).`;
    return value;
  }

  function shouldSkip(node) {
    const parent = node.parentElement;
    if (!parent) return false;
    return !!parent.closest('#patternReview .pill, textarea, input, option, script, style');
  }

  function translateTextNode(node) {
    if (node.nodeType !== Node.TEXT_NODE || shouldSkip(node)) return;
    if (!originalText.has(node)) originalText.set(node, node.nodeValue);
    const original = originalText.get(node);
    const next = language === 'en' ? original : translateString(original);
    if (node.nodeValue !== next) node.nodeValue = next;
  }

  function translateElement(el) {
    if (!(el instanceof Element)) return;
    if (el.matches('input[placeholder], textarea[placeholder]')) {
      if (!originalPlaceholder.has(el)) originalPlaceholder.set(el, el.getAttribute('placeholder') || '');
      const original = originalPlaceholder.get(el);
      el.setAttribute('placeholder', language === 'en' ? original : translateString(original));
    }
    if (el.hasAttribute('aria-label')) {
      if (!originalAria.has(el)) originalAria.set(el, el.getAttribute('aria-label') || '');
      const original = originalAria.get(el);
      el.setAttribute('aria-label', language === 'en' ? original : translateString(original));
    }
    for (const child of el.childNodes) {
      if (child.nodeType === Node.TEXT_NODE) translateTextNode(child);
      else if (child.nodeType === Node.ELEMENT_NODE) translateElement(child);
    }
  }

  function applyLanguage(nextLanguage) {
    language = normalizedLanguage(nextLanguage);
    document.documentElement.lang = language;
    applying = true;
    translateElement(document.body);
    const select = document.getElementById('heartlightLanguage');
    if (select) select.value = language;
    applying = false;
    try { localStorage.setItem(STORAGE_KEY, language); } catch {}
    document.dispatchEvent(new CustomEvent('heartlight:languagechange', { detail: { language } }));
  }

  function installSelector() {
    if (document.getElementById('heartlightLanguage')) return;
    const header = document.querySelector('header.top');
    if (!header) return;
    const box = document.createElement('div');
    box.className = 'status';
    box.style.display = 'flex';
    box.style.gap = '8px';
    box.style.alignItems = 'center';
    box.style.flexWrap = 'wrap';
    box.innerHTML = '<label for="heartlightLanguage" style="font-weight:700;margin:0">🌐 <span id="heartlightLanguageLabel">Language</span></label><select id="heartlightLanguage" style="width:auto;min-width:140px;padding:7px 10px"><option value="en">English</option><option value="es">Español</option></select>';
    header.appendChild(box);
    box.querySelector('select').addEventListener('change', event => applyLanguage(event.target.value));
    originalText.set(box.querySelector('#heartlightLanguageLabel').firstChild, 'Language');
    es.set('Language', 'Idioma');
  }

  const nativeAlert = window.alert.bind(window);
  const nativeConfirm = window.confirm.bind(window);
  window.alert = message => nativeAlert(translateString(String(message)));
  window.confirm = message => nativeConfirm(translateString(String(message)));

  function start() {
    installSelector();
    let saved = '';
    try { saved = localStorage.getItem(STORAGE_KEY) || ''; } catch {}
    applyLanguage(saved || navigator.language || 'en');

    const observer = new MutationObserver(records => {
      if (applying || language === 'en') return;
      applying = true;
      for (const record of records) {
        if (record.type === 'characterData') translateTextNode(record.target);
        for (const node of record.addedNodes) {
          if (node.nodeType === Node.TEXT_NODE) translateTextNode(node);
          else if (node.nodeType === Node.ELEMENT_NODE) translateElement(node);
        }
      }
      applying = false;
    });
    observer.observe(document.body, { subtree: true, childList: true, characterData: true });
  }

  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', start, { once: true });
  else start();
})();

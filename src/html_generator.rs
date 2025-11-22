use crate::models::B50Project;
use std::fs::File;
use std::io::Write;

pub struct HtmlGenerator;

impl HtmlGenerator {
    pub fn generate(project: &B50Project, output_path: &str) -> std::io::Result<()> {
        let data_json = serde_json::to_string(&project.records).unwrap();
        let player_name = &project.player_name;
        let generated_at = &project.generated_at;

        let mut html = String::new();
        html.push_str(&format!(r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Paradigm: Reboot B50</title>
    <style>
        /* Base & Typography */
        :root {{
            --bg-color: #08080c;
            --text-main: #ffffff;
            --text-sub: #a0a0a0;
            --accent-blue: #3fcbff;
            --accent-purple: #a38bf3;
            --panel-bg: rgba(255, 255, 255, 0.05);
            --panel-active: rgba(163, 139, 243, 0.2);
        }}
        body {{
            margin: 0; padding: 0;
            background-color: var(--bg-color);
            color: var(--text-main);
            font-family: 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            overflow: hidden;
            user-select: none;
        }}

        /* Background Art */
        #bg-layer {{
            position: absolute; top: 0; left: 0; width: 100vw; height: 100vh;
            background-size: cover; background-position: center;
            filter: blur(30px) brightness(0.3);
            transition: background-image 0.5s ease;
            z-index: -1;
        }}
        #grid-overlay {{
            position: absolute; top: 0; left: 0; width: 100%; height: 100%;
            background-image: 
                linear-gradient(rgba(255,255,255,0.03) 1px, transparent 1px),
                linear-gradient(90deg, rgba(255,255,255,0.03) 1px, transparent 1px);
            background-size: 40px 40px;
            z-index: -1;
        }}

        /* Layout */
        #app {{
            display: flex;
            width: 100vw; height: 100vh;
            position: relative;
        }}

        /* Left Sidebar (Song List) */
        #sidebar {{
            width: 35%;
            height: 100%;
            overflow-y: auto;
            padding: 20px 0 20px 20px;
            box-sizing: border-box;
            scrollbar-width: none;
        }}
        #sidebar::-webkit-scrollbar {{ display: none; }}

        .list-item {{
            display: flex;
            align-items: center;
            height: 70px;
            margin-bottom: 8px;
            padding: 0 20px;
            background: var(--panel-bg);
            /* Angled cut style */
            clip-path: polygon(0 0, 100% 0, 95% 100%, 0 100%);
            transition: all 0.3s cubic-bezier(0.2, 0.8, 0.2, 1);
            cursor: pointer;
            position: relative;
            border-left: 4px solid transparent;
            
            /* Focus effect: blur non-active items */
            filter: blur(6px) grayscale(0.3);
            opacity: 0.6;
        }}
        
        .list-item:hover {{
            filter: blur(0.5px) grayscale(0.1);
            opacity: 0.8;
        }}
        
        .list-item.active {{
            background: linear-gradient(90deg, var(--panel-active), transparent);
            border-left: 4px solid var(--accent-purple);
            transform: translateX(15px);
            
            /* Reset focus effect for active item */
            filter: none;
            opacity: 1;
        }}

        .item-rank {{
            font-size: 24px; font-weight: bold; color: rgba(255,255,255,0.2);
            width: 50px;
            font-family: 'Impact', sans-serif;
        }}
        .list-item.active .item-rank {{ color: var(--accent-purple); }}

        /* Section headers inside sidebar */
        .section-header {{ padding: 10px 20px; color: var(--text-sub); font-size: 13px; font-weight: 700; opacity: 0.9; }}
        .section-title {{ font-size: 14px; color: var(--accent-blue); margin-bottom: 6px; }}

        .item-info {{ flex: 1; overflow: hidden; }}
        .item-title {{ 
            font-size: 16px; font-weight: 600; 
            white-space: nowrap; overflow: hidden; text-overflow: ellipsis;
        }}
        .item-meta {{
            font-size: 12px; color: var(--text-sub);
            display: flex; gap: 10px; margin-top: 2px;
        }}

        .diff-badge {{
            font-size: 10px; padding: 2px 6px; border-radius: 2px;
            font-weight: bold; text-transform: uppercase;
            color: #000;
        }}

        /* Right Content (Detail View) */
        #detail-view {{
            flex: 1;
            padding: 40px;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            position: relative;
        }}

        /* Header Info */
        .header-info {{
            position: absolute; top: 40px; right: 60px;
            text-align: right;
        }}
        .player-name {{ font-size: 14px; color: var(--text-sub); letter-spacing: 2px; }}
        .total-rating {{ font-size: 32px; font-weight: bold; color: var(--accent-blue); }}

        /* Main Cover Art */
        .cover-wrapper {{
            position: relative;
            width: 450px; height: 450px;
            margin-bottom: 30px;
        }}
        .cover-image {{
            width: 100%; height: 100%;
            object-fit: cover;
            border: 2px solid rgba(255,255,255,0.2);
            box-shadow: 0 0 30px rgba(0,0,0,0.5);
            /* Inner decoration lines mimic game UI */
            position: relative;
            z-index: 1;
        }}
        #cover-decor {{
            position: absolute;
            border: 1px solid;
            width: 104%; height: 104%;
            top: -2%; left: -2%;
            z-index: 0;
            opacity: 0.5;
        }}

        /* Song Details */
        .song-info {{ text-align: center; width: 100%; max-width: 600px; }}
        
        .detail-title-box {{
            overflow: hidden;
            white-space: nowrap;
            margin-bottom: 5px;
        }}
        .detail-title {{
            font-size: 48px; font-weight: 800;
            text-shadow: 0 2px 10px rgba(0,0,0,0.5);
            display: inline-block;
        }}

        .detail-artist {{
            font-size: 20px; color: var(--accent-blue);
            margin-bottom: 30px;
            letter-spacing: 1px;
        }}

        /* Metrics Grid */
        .metrics-container {{
            display: flex;
            justify-content: center;
            gap: 40px;
            background: linear-gradient(90deg, transparent, rgba(0,0,0,0.6), transparent);
            padding: 20px 0;
            width: 100%;
        }}
        
        .metric-group {{ text-align: center; }}
        .metric-label {{ 
            font-size: 12px; color: var(--text-sub); 
            letter-spacing: 2px; margin-bottom: 5px; 
        }}
        .metric-value {{ font-size: 36px; font-weight: bold; font-family: 'Impact', sans-serif; }}
        .metric-sub {{ font-size: 16px; margin-left: 5px; color: var(--text-sub); }}

        /* Progress Bar */
        #progress-bar {{
            position: absolute; bottom: 0; left: 0;
            height: 4px; width: 0%;
            background: var(--accent-purple);
            box-shadow: 0 0 10px var(--accent-purple);
            transition: width linear;
        }}

        /* Pause Overlay */
        #status-overlay {{
            position: absolute; top: 20px; left: 40%;
            background: #000; color: #fff;
            padding: 5px 10px; font-size: 12px;
            display: none; letter-spacing: 2px;
            border: 1px solid #fff;
        }}

        /* Animations */
        .fade-in {{ animation: fadeIn 0.4s ease-out forwards; }}
        @keyframes fadeIn {{
            from {{ opacity: 0; transform: translateY(10px); }}
            to {{ opacity: 1; transform: translateY(0); }}
        }}
        /* Comment Panel (inline editable, parallelogram to match list) */
        .comment-panel {{
            width: 680px;
            background: var(--panel-bg);
            /* Angled cut style like left list */
            clip-path: polygon(0 0, 100% 0, 95% 100%, 0 100%);
            display: flex;
            flex-direction: column;
            align-items: flex-start;
            padding: 16px 22px 16px 18px;
            margin-top: 24px;
            color: var(--text-main);
            box-sizing: border-box;
            box-shadow: 0 8px 30px rgba(0,0,0,0.6);
        }}
        .comment-header {{ width: 100%; text-align: left; }}

        #comment-display {{
            min-width: 100%;
        }}

        .comment-text {{
            margin-top: 12px;
            color: var(--text-main);
            white-space: pre-wrap;
            outline: none;
            text-align: left;
            min-height: 64px;
            cursor: text;
            user-select: text;
            caret-color: var(--accent-blue);
            font-size: 14px;
        }}
    </style>
</head>
<body>
    <div id="bg-layer"></div>
    <div id="grid-overlay"></div>

    <div id="app">
        <!-- Left Sidebar -->
        <div id="sidebar">
            <!-- Generated by JS -->
        </div>

        <!-- Right Content -->
        <div id="detail-view">
            <div class="header-info">
                <div class="player-name" contenteditable="true">{0}</div>
                <div class="total-rating">BEST 50</div>
                <div style="font-size: 12px; opacity: 0.5;">{1}</div>
            </div>

            <div class="cover-wrapper fade-in" id="anim-cover">
                <div id="cover-decor"></div>
                <img src="" class="cover-image" id="detail-cover">
            </div>

            <div class="song-info fade-in" id="anim-info">
                <div class="detail-title-box">
                    <div class="detail-title" id="detail-title">Title</div>
                </div>
                <div class="detail-artist" id="detail-artist">Artist</div>
                
                <div class="metrics-container">
                    <div class="metric-group">
                        <div class="metric-label">RATING</div>
                        <div class="metric-value" id="detail-rating">0.00</div>
                    </div>
                    <div class="metric-group">
                        <div class="metric-label">SCORE</div>
                        <div class="metric-value" id="detail-score">0000000</div>
                    </div>
                    <div class="metric-group">
                        <div class="metric-label">ACCURACY</div>
                        <div class="metric-value"><span id="detail-acc">0.00</span><span class="metric-sub">%</span></div>
                    </div>
                    <div class="metric-group">
                        <div class="metric-label">LEVEL</div>
                        <div class="metric-value" id="detail-level">0.0</div>
                    </div>
                </div>
                </div>
                <!-- Comment Panel (inline editable, saved to localStorage) - placed below metrics -->
                <div class="comment-panel" id="comment-panel">
                    <div class="comment-header">
                        <div style="font-weight:700;">Comments</div>
                    </div>
                    <div id="comment-display" class="comment-text" contenteditable="true"></div>
                </div>
            </div>
        </div>
        <div id="progress-bar"></div>
        <div id="status-overlay">PAUSED</div>
    </div>

    <script>
        // Data Injection
        const records = {2};
        
        // Config
        const DURATION_PER_SLIDE = 5000; // (unused) previously used for auto-advance

        // State
        let currentIndex = 0;
        // displayedOrder: array of record indices in the order items are shown in the sidebar
        let displayedOrder = [];
        let currentDisplayPos = 0;

        // DOM Elements
        const elSidebar = document.getElementById('sidebar');
        const elBg = document.getElementById('bg-layer');
        const elTitle = document.getElementById('detail-title');
        const elArtist = document.getElementById('detail-artist');
        const elCover = document.getElementById('detail-cover');
        const elCoverDecor = document.getElementById('cover-decor');
        const elRating = document.getElementById('detail-rating');
        const elScore = document.getElementById('detail-score');
        const elAcc = document.getElementById('detail-acc');
        const elLevel = document.getElementById('detail-level');
        const elProgress = document.getElementById('progress-bar');
        const elStatus = document.getElementById('status-overlay');
        const elAnimCover = document.getElementById('anim-cover');
        const elAnimInfo = document.getElementById('anim-info');

        // Comment elements (inline editable)
        const elCommentPanel = document.getElementById('comment-panel');
        const elCommentDisplay = document.getElementById('comment-display');

        // Comments persistence (localStorage)
        const COMMENTS_KEY = 'b50_comments_v1';
        let comments = {{}};

        function loadComments() {{
            try {{
                const raw = localStorage.getItem(COMMENTS_KEY);
                if (raw) comments = JSON.parse(raw) || {{}};
            }} catch (e) {{
                comments = {{}};
            }}
            // merge into records for convenience
            records.forEach((r, i) => {{ if (comments.hasOwnProperty(i)) r.comment = comments[i]; }});
        }}

        function saveCommentsToStorage() {{
            try {{
                localStorage.setItem(COMMENTS_KEY, JSON.stringify(comments));
            }} catch (e) {{ /* ignore */ }}
        }}

        // render comment into contenteditable area without triggering input save
        let _isSettingComment = false;
        function renderComment(index) {{
            const rec = records[index];
            const c = rec.comment || comments[index] || '';
            _isSettingComment = true;
            elCommentDisplay.innerText = c;
            // clear flag next tick
            setTimeout(() => (_isSettingComment = false), 0);
        }}

        function debounce(fn, wait) {{
            let t = null;
            return function(...args) {{
                if (t) clearTimeout(t);
                t = setTimeout(() => fn.apply(this, args), wait);
            }};
        }}

        const debouncedSave = debounce(() => saveCommentsToStorage(), 250);

        elCommentDisplay.addEventListener('input', () => {{
            if (_isSettingComment) return;
            const text = elCommentDisplay.innerText.trim();
            if (text) {{
                comments[currentIndex] = text;
                records[currentIndex].comment = text;
            }} else {{
                delete comments[currentIndex];
                records[currentIndex].comment = null;
            }}
            debouncedSave();
        }});

        elCommentDisplay.addEventListener('blur', () => saveCommentsToStorage());

        // Helper: Difficulty Colors
        function getDiffColor(diff) {{
            switch(diff) {{
                case 'Detected': return '#3fcbff';
                case 'Invaded': return '#ff6b6b';
                case 'Massive': return '#8f629d';
                default: return '#888';
            }}
        }}

        // Init Sidebar: split into New (Best 15) and Old (Best 35)
        function initSidebar() {{
            elSidebar.innerHTML = '';
            displayedOrder = [];

            // Collect indices for new and old tracks (records are sorted by rating desc)
            const newIndices = [];
            const oldIndices = [];
            for (let i = 0; i < records.length; i++) {{
                const rec = records[i];
                if (rec.song_metadata && rec.song_metadata.is_new) newIndices.push(i);
                else oldIndices.push(i);
            }}

            const topNew = newIndices.slice(0, 15);
            const topOld = oldIndices.slice(0, 35);

            // Render New section header
            const newHeader = document.createElement('div');
            newHeader.className = 'section-header';
            newHeader.innerHTML = `<div class="section-title">Best 15</div>`;
            elSidebar.appendChild(newHeader);

            // Render new items (show from 15 -> 1)
            for (let k = topNew.length - 1; k >= 0; k--) {{
                const idx = topNew[k];
                const rec = records[idx];
                const item = document.createElement('div');
                item.className = 'list-item';
                item.id = `item-${{idx}}`;
                item.onclick = () => jumpTo(idx);

                const diffColor = getDiffColor(rec.difficulty);
                const rankLabel = k + 1; // reversed: length..1 -> display 15..1

                item.innerHTML = `
                    <div class="item-rank">#${{rankLabel}}</div>
                    <div class="item-info">
                        <div class="item-title">${{rec.song_metadata.title}}</div>
                        <div class="item-meta">
                            <span class="diff-badge" style="background:${{diffColor}}">${{rec.difficulty}} ${{rec.level.toFixed(1)}}</span>
                            <span>${{rec.score}}</span>
                        </div>
                    </div>
                `;
                elSidebar.appendChild(item);
                displayedOrder.push(idx);
            }}

            // Separator for old section
            const oldHeader = document.createElement('div');
            oldHeader.className = 'section-header';
            oldHeader.innerHTML = `<div class="section-title">Best 35</div>`;
            elSidebar.appendChild(oldHeader);

            // Render old items (show from 35 -> 1)
            for (let k = topOld.length - 1; k >= 0; k--) {{
                const idx = topOld[k];
                const rec = records[idx];
                const item = document.createElement('div');
                item.className = 'list-item';
                item.id = `item-${{idx}}`;
                item.onclick = () => jumpTo(idx);

                const diffColor = getDiffColor(rec.difficulty);
                const rankLabel = k + 1; // reversed within group: e.g., 35..1

                item.innerHTML = `
                    <div class="item-rank">#${{rankLabel}}</div>
                    <div class="item-info">
                        <div class="item-title">${{rec.song_metadata.title}}</div>
                        <div class="item-meta">
                            <span class="diff-badge" style="background:${{diffColor}}">${{rec.difficulty}} ${{rec.level.toFixed(1)}}</span>
                            <span>${{rec.score}}</span>
                        </div>
                    </div>
                `;
                elSidebar.appendChild(item);
                displayedOrder.push(idx);
            }}
        }}

        // Main Render Function
        function render(index) {{
            if (index >= records.length) index = 0;
            currentIndex = index;
            const rec = records[index];
            
            // 1. Update Sidebar Active State
            document.querySelectorAll('.list-item').forEach(el => el.classList.remove('active'));
            const activeItem = document.getElementById(`item-${{index}}`);
            if (activeItem) {{
                activeItem.classList.add('active');
                // Auto Scroll Sidebar
                activeItem.scrollIntoView({{ behavior: 'smooth', block: 'center' }});
            }}

            // 2. Update Background
            elBg.style.backgroundImage = `url('${{rec.song_metadata.cover_url}}')`;

            // 3. Update Right Details with Animation Reset
            elAnimCover.classList.remove('fade-in');
            elAnimInfo.classList.remove('fade-in');
            void elAnimCover.offsetWidth; // trigger reflow
            elAnimCover.classList.add('fade-in');
            elAnimInfo.classList.add('fade-in');

            elCover.src = rec.song_metadata.cover_url;
            elCover.style.borderColor = getDiffColor(rec.difficulty);
            elCoverDecor.style.borderColor = getDiffColor(rec.difficulty);
            elTitle.textContent = rec.song_metadata.title;
            elArtist.textContent = rec.song_metadata.artist;
            elRating.textContent = rec.rating.toFixed(2);
            elScore.textContent = rec.score;
            elAcc.textContent = rec.acc.toFixed(2);
            
            const diffColor = getDiffColor(rec.difficulty);
            elLevel.textContent = rec.level.toFixed(1);
            elLevel.style.color = diffColor;

            // 4. Handle Title Scrolling
            handleTitleScroll(elTitle);

            // 5. Render comment for this slide (if present)
            if (typeof renderComment === 'function') renderComment(index);
        }}

        function handleTitleScroll(element) {{
            // Reset animation
            element.style.animation = 'none';
            element.style.transform = 'translateX(0)';
            
            const container = element.parentElement;
            if (element.scrollWidth > container.clientWidth) {{
                const distance = element.scrollWidth - container.clientWidth + 20;
                const duration = distance / 50; // pixels per sec
                
                const keyframes = `
                    @keyframes scroll-${{currentIndex}} {{
                        0% {{ transform: translateX(0); }}
                        20% {{ transform: translateX(0); }}
                        80% {{ transform: translateX(-${{distance}}px); }}
                        100% {{ transform: translateX(-${{distance}}px); }}
                    }}
                `;
                
                let style = document.getElementById('dyn-style');
                if (!style) {{
                    style = document.createElement('style');
                    style.id = 'dyn-style';
                    document.head.appendChild(style);
                }}
                style.innerHTML = keyframes;
                
                element.style.animation = `scroll-${{currentIndex}} ${{Math.max(duration, 4)}}s linear infinite alternate`;
            }}
        }}

        // Navigation helpers (no auto-advance)
        function getDisplayPosFromRecordIndex(recIdx) {{
            return displayedOrder.indexOf(recIdx);
        }}

        function nextTrack() {{
            if (!displayedOrder || displayedOrder.length === 0) return;
            let pos = getDisplayPosFromRecordIndex(currentIndex);
            if (pos === -1) pos = 0; // fallback
            if (pos >= displayedOrder.length - 1) return;
            const nextRec = displayedOrder[pos + 1];
            render(nextRec);
        }}

        function prevTrack() {{
            if (!displayedOrder || displayedOrder.length === 0) return;
            let pos = getDisplayPosFromRecordIndex(currentIndex);
            if (pos === -1) pos = 0;
            if (pos <= 0) return;
            const prevRec = displayedOrder[pos - 1];
            render(prevRec);
        }}

        function jumpTo(index) {{
            render(index);
        }}

        // Controls: Left/Right arrows to navigate
        document.addEventListener('keydown', (e) => {{
            if (e.code === 'ArrowLeft') {{
                e.preventDefault();
                prevTrack();
            }} else if (e.code === 'ArrowRight') {{
                e.preventDefault();
                nextTrack();
            }}
        }});

        // Init
        window.onload = () => {{
            // load persisted comments before initializing UI
            if (typeof loadComments === 'function') loadComments();
            initSidebar();
            // If URL contains ?i=<index>, render that slide and do not auto-start timer.
            const params = new URLSearchParams(window.location.search);
            const idxParam = params.get('i');
            // Determine default index: prefer Best15's 15th song (B15 #15). If not enough new songs,
            // fall back to the last new song; if no new songs, fall back to last record.
            const newAll = [];
            for (let i = 0; i < records.length; i++) {{ if (records[i].song_metadata && records[i].song_metadata.is_new) newAll.push(i); }}
            let defaultIndex = Math.max(records.length - 1, 0);
            if (newAll.length >= 15) {{
                defaultIndex = newAll[14];
            }} else if (newAll.length > 0) {{
                defaultIndex = newAll[newAll.length - 1];
            }}

            if (idxParam !== null) {{
                const idx = parseInt(idxParam, 10);
                if (!isNaN(idx) && idx >= 0 && idx < records.length) {{
                    render(idx);
                }} else {{
                    render(defaultIndex);
                }}
            }} else {{
                // default: render B15's #15 (or fallback)
                render(defaultIndex);
            }}
        }};
    </script>
</body>
</html>
"#, player_name, generated_at, data_json));

        let mut file = File::create(output_path)?;
        file.write_all(html.as_bytes())?;
        Ok(())
    }
}

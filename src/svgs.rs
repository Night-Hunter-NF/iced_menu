use iced_native::widget::svg;

pub fn close_svg() -> svg::Handle {
    svg::Handle::from_memory(
        "<svg xmlns='http://www.w3.org/2000/svg' fill='none' stroke='#000' viewBox='0 0 64 64'>
    <path d='m16 16 32 32m0-32L16 48'/>
  </svg>
  ".as_bytes(),
    )
}
pub fn minimize_svg() -> svg::Handle {
    svg::Handle::from_memory(
        
        "
        <svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 200 200'>
        <line x1='70' y1='100' x2='130' y2='100' stroke='black' stroke-width='3' />
      </svg>
  ".as_bytes(),
    )
}
pub fn restore() -> svg::Handle {
    svg::Handle::from_memory(
        "
        <svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 150 270'>
        <line x1='7' y1='200' x2='7' y2='100' stroke='black' stroke-width='2' />
        <line x1='6' y1='100' x2='106' y2='101'  stroke='black' stroke-width='2'/>
        <line x1='106' y1='100' x2='106' y2='200'  stroke='black' stroke-width='2'/>
        <line x1='107' y1='200' x2='6' y2='200'  stroke='black' stroke-width='2'/>
        <line x1='31' y1='100' x2='31' y2='75'  stroke='black' stroke-width='2'/>
        <line x1='30' y1='75' x2='133' y2='75'  stroke='black' stroke-width='2'/>
        <line x1='131' y1='75' x2='131' y2='175'  stroke='black' stroke-width='2'/>
        <line x1='132' y1='175' x2='106' y2='175'  stroke='black' stroke-width='2'/>
      </svg>
      ".as_bytes()
,
    )
}


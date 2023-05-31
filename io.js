export const detectButtonClickEvent = async id => {
  await new Promise(resolve => {
    document.getElementById(id).addEventListener('click', () => {
      resolve();
    });
  });
};

export const cleanOutput = () => {
  output.setValue('');
};

export const outputChar = c => {
  output.setValue(output.getValue() + c);
};

export const outputStr = str => {
  output.setValue(str);
};

export const showProgram = str => {
  const programDiv = document.getElementById('program');
  programDiv.innerHTML = str;
};

export const markUpProgram = (program, idx) => {
  const programDiv = document.getElementById('program');
  programDiv.innerHTML =
    program.substring(0, idx) +
    '<span class="red">' +
    program.charAt(idx) +
    '</span>' +
    program.substring(idx + 1);
};

export const cleanMemoryMap = () => {
  const memoryMap = document.querySelectorAll('.memory');
  memoryMap.forEach(m => {
    m.innerHTML = '000';
    m.classList.remove('red');
  });
};

export const getMemDisplayRadix = () => {
  return Array.from(
    document.querySelectorAll(`input[name='mem-display']`)
  ).find(elm => elm.checked).value;
};

export const writeMemoryMap = (idx, value, radix) => {
  const memoryMap = document.querySelectorAll('.memory');
  if (radix === 10) {
    memoryMap[idx].innerHTML = String(value).padStart(3, '0');
  } else if (radix === 16) {
    memoryMap[idx].innerHTML = value
      .toString(16)
      .toUpperCase()
      .padStart(2, '0');
  }

  memoryMap.forEach(m => m.classList.remove('red'));
  memoryMap[idx].classList.add('red');
};

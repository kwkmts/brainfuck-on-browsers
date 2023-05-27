export const detectButtonClickEvent = async id => {
  await new Promise(resolve => {
    document.getElementById(id).addEventListener('click', () => {
      resolve();
    });
  });
};

export const cleanConsole = () => {
  console.setValue('');
};

export const outputChar = c => {
  console.setValue(console.getValue() + c);
};

export const outputStr = str => {
  console.setValue(str);
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

export const writeMemoryMap = (idx, value) => {
  const memoryMap = document.querySelectorAll('.memory');
  memoryMap[idx].innerHTML = String(value).padStart(3, '0');
  memoryMap.forEach(m => m.classList.remove('red'));
  memoryMap[idx].classList.add('red');
};

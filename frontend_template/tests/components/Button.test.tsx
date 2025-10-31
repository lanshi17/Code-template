import React from 'react';
import { render, screen } from '@testing-library/react';
import { Button } from '../../src/components/common/Button';

test('renders button', () => {
  render(<Button>Click me</Button>);
  const buttonElement = screen.getByText(/click me/i);
  expect(buttonElement).toBeInTheDocument();
});

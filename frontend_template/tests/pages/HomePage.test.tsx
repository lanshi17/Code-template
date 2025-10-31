import React from 'react';
import { render, screen } from '@testing-library/react';
import HomePage from '../../src/pages/HomePage';

test('renders home page', () => {
  render(<HomePage />);
  const linkElement = screen.getByText(/home page/i);
  expect(linkElement).toBeInTheDocument();
});

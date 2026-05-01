#[doc = "Register `SGPIO14` reader"]
pub type R = crate::R<Sgpio14Spec>;
#[doc = "Register `SGPIO14` writer"]
pub type W = crate::W<Sgpio14Spec>;
#[doc = "Field `InputValueOfSGPIO128` reader - Input value of SGPIO_128"]
pub type InputValueOfSgpio128R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO129` reader - Input value of SGPIO_129"]
pub type InputValueOfSgpio129R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO130` reader - Input value of SGPIO_130"]
pub type InputValueOfSgpio130R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO131` reader - Input value of SGPIO_131"]
pub type InputValueOfSgpio131R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO132` reader - Input value of SGPIO_132"]
pub type InputValueOfSgpio132R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO133` reader - Input value of SGPIO_133"]
pub type InputValueOfSgpio133R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO134` reader - Input value of SGPIO_134"]
pub type InputValueOfSgpio134R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO135` reader - Input value of SGPIO_135"]
pub type InputValueOfSgpio135R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO136` reader - Input value of SGPIO_136"]
pub type InputValueOfSgpio136R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO137` reader - Input value of SGPIO_137"]
pub type InputValueOfSgpio137R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO138` reader - Input value of SGPIO_138"]
pub type InputValueOfSgpio138R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO139` reader - Input value of SGPIO_139"]
pub type InputValueOfSgpio139R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO140` reader - Input value of SGPIO_140"]
pub type InputValueOfSgpio140R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO141` reader - Input value of SGPIO_141"]
pub type InputValueOfSgpio141R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO142` reader - Input value of SGPIO_142"]
pub type InputValueOfSgpio142R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO143` reader - Input value of SGPIO_143"]
pub type InputValueOfSgpio143R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO144` reader - Input value of SGPIO_144"]
pub type InputValueOfSgpio144R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO145` reader - Input value of SGPIO_145"]
pub type InputValueOfSgpio145R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO146` reader - Input value of SGPIO_146"]
pub type InputValueOfSgpio146R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO147` reader - Input value of SGPIO_147"]
pub type InputValueOfSgpio147R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO148` reader - Input value of SGPIO_148"]
pub type InputValueOfSgpio148R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO149` reader - Input value of SGPIO_149"]
pub type InputValueOfSgpio149R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO150` reader - Input value of SGPIO_150"]
pub type InputValueOfSgpio150R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO151` reader - Input value of SGPIO_151"]
pub type InputValueOfSgpio151R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO152` reader - Input value of SGPIO_152"]
pub type InputValueOfSgpio152R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO153` reader - Input value of SGPIO_153"]
pub type InputValueOfSgpio153R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO154` reader - Input value of SGPIO_154"]
pub type InputValueOfSgpio154R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO155` reader - Input value of SGPIO_155"]
pub type InputValueOfSgpio155R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO156` reader - Input value of SGPIO_156"]
pub type InputValueOfSgpio156R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO157` reader - Input value of SGPIO_157"]
pub type InputValueOfSgpio157R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO158` reader - Input value of SGPIO_158"]
pub type InputValueOfSgpio158R = crate::BitReader;
#[doc = "Field `InputValueOfSGPIO159` reader - Input value of SGPIO_159"]
pub type InputValueOfSgpio159R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Input value of SGPIO_128"]
    #[inline(always)]
    pub fn input_value_of_sgpio128(&self) -> InputValueOfSgpio128R {
        InputValueOfSgpio128R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input value of SGPIO_129"]
    #[inline(always)]
    pub fn input_value_of_sgpio129(&self) -> InputValueOfSgpio129R {
        InputValueOfSgpio129R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input value of SGPIO_130"]
    #[inline(always)]
    pub fn input_value_of_sgpio130(&self) -> InputValueOfSgpio130R {
        InputValueOfSgpio130R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input value of SGPIO_131"]
    #[inline(always)]
    pub fn input_value_of_sgpio131(&self) -> InputValueOfSgpio131R {
        InputValueOfSgpio131R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input value of SGPIO_132"]
    #[inline(always)]
    pub fn input_value_of_sgpio132(&self) -> InputValueOfSgpio132R {
        InputValueOfSgpio132R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input value of SGPIO_133"]
    #[inline(always)]
    pub fn input_value_of_sgpio133(&self) -> InputValueOfSgpio133R {
        InputValueOfSgpio133R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input value of SGPIO_134"]
    #[inline(always)]
    pub fn input_value_of_sgpio134(&self) -> InputValueOfSgpio134R {
        InputValueOfSgpio134R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input value of SGPIO_135"]
    #[inline(always)]
    pub fn input_value_of_sgpio135(&self) -> InputValueOfSgpio135R {
        InputValueOfSgpio135R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Input value of SGPIO_136"]
    #[inline(always)]
    pub fn input_value_of_sgpio136(&self) -> InputValueOfSgpio136R {
        InputValueOfSgpio136R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input value of SGPIO_137"]
    #[inline(always)]
    pub fn input_value_of_sgpio137(&self) -> InputValueOfSgpio137R {
        InputValueOfSgpio137R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Input value of SGPIO_138"]
    #[inline(always)]
    pub fn input_value_of_sgpio138(&self) -> InputValueOfSgpio138R {
        InputValueOfSgpio138R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Input value of SGPIO_139"]
    #[inline(always)]
    pub fn input_value_of_sgpio139(&self) -> InputValueOfSgpio139R {
        InputValueOfSgpio139R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Input value of SGPIO_140"]
    #[inline(always)]
    pub fn input_value_of_sgpio140(&self) -> InputValueOfSgpio140R {
        InputValueOfSgpio140R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input value of SGPIO_141"]
    #[inline(always)]
    pub fn input_value_of_sgpio141(&self) -> InputValueOfSgpio141R {
        InputValueOfSgpio141R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Input value of SGPIO_142"]
    #[inline(always)]
    pub fn input_value_of_sgpio142(&self) -> InputValueOfSgpio142R {
        InputValueOfSgpio142R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Input value of SGPIO_143"]
    #[inline(always)]
    pub fn input_value_of_sgpio143(&self) -> InputValueOfSgpio143R {
        InputValueOfSgpio143R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Input value of SGPIO_144"]
    #[inline(always)]
    pub fn input_value_of_sgpio144(&self) -> InputValueOfSgpio144R {
        InputValueOfSgpio144R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Input value of SGPIO_145"]
    #[inline(always)]
    pub fn input_value_of_sgpio145(&self) -> InputValueOfSgpio145R {
        InputValueOfSgpio145R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input value of SGPIO_146"]
    #[inline(always)]
    pub fn input_value_of_sgpio146(&self) -> InputValueOfSgpio146R {
        InputValueOfSgpio146R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Input value of SGPIO_147"]
    #[inline(always)]
    pub fn input_value_of_sgpio147(&self) -> InputValueOfSgpio147R {
        InputValueOfSgpio147R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Input value of SGPIO_148"]
    #[inline(always)]
    pub fn input_value_of_sgpio148(&self) -> InputValueOfSgpio148R {
        InputValueOfSgpio148R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Input value of SGPIO_149"]
    #[inline(always)]
    pub fn input_value_of_sgpio149(&self) -> InputValueOfSgpio149R {
        InputValueOfSgpio149R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Input value of SGPIO_150"]
    #[inline(always)]
    pub fn input_value_of_sgpio150(&self) -> InputValueOfSgpio150R {
        InputValueOfSgpio150R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Input value of SGPIO_151"]
    #[inline(always)]
    pub fn input_value_of_sgpio151(&self) -> InputValueOfSgpio151R {
        InputValueOfSgpio151R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Input value of SGPIO_152"]
    #[inline(always)]
    pub fn input_value_of_sgpio152(&self) -> InputValueOfSgpio152R {
        InputValueOfSgpio152R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Input value of SGPIO_153"]
    #[inline(always)]
    pub fn input_value_of_sgpio153(&self) -> InputValueOfSgpio153R {
        InputValueOfSgpio153R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Input value of SGPIO_154"]
    #[inline(always)]
    pub fn input_value_of_sgpio154(&self) -> InputValueOfSgpio154R {
        InputValueOfSgpio154R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Input value of SGPIO_155"]
    #[inline(always)]
    pub fn input_value_of_sgpio155(&self) -> InputValueOfSgpio155R {
        InputValueOfSgpio155R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Input value of SGPIO_156"]
    #[inline(always)]
    pub fn input_value_of_sgpio156(&self) -> InputValueOfSgpio156R {
        InputValueOfSgpio156R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Input value of SGPIO_157"]
    #[inline(always)]
    pub fn input_value_of_sgpio157(&self) -> InputValueOfSgpio157R {
        InputValueOfSgpio157R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Input value of SGPIO_158"]
    #[inline(always)]
    pub fn input_value_of_sgpio158(&self) -> InputValueOfSgpio158R {
        InputValueOfSgpio158R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Input value of SGPIO_159"]
    #[inline(always)]
    pub fn input_value_of_sgpio159(&self) -> InputValueOfSgpio159R {
        InputValueOfSgpio159R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Debug Serial In Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio14Spec;
impl crate::RegisterSpec for Sgpio14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio14::R`](R) reader structure"]
impl crate::Readable for Sgpio14Spec {}
#[doc = "`write(|w| ..)` method takes [`sgpio14::W`](W) writer structure"]
impl crate::Writable for Sgpio14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO14 to value 0"]
impl crate::Resettable for Sgpio14Spec {}

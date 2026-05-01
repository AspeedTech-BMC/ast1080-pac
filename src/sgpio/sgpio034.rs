#[doc = "Register `SGPIO034` reader"]
pub type R = crate::R<Sgpio034Spec>;
#[doc = "Register `SGPIO034` writer"]
pub type W = crate::W<Sgpio034Spec>;
#[doc = "Field `OutputValueOfSGPIO128` reader - Output value of SGPIO_128"]
pub type OutputValueOfSgpio128R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO129` reader - Output value of SGPIO_129"]
pub type OutputValueOfSgpio129R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO130` reader - Output value of SGPIO_130"]
pub type OutputValueOfSgpio130R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO131` reader - Output value of SGPIO_131"]
pub type OutputValueOfSgpio131R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO132` reader - Output value of SGPIO_132"]
pub type OutputValueOfSgpio132R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO133` reader - Output value of SGPIO_133"]
pub type OutputValueOfSgpio133R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO134` reader - Output value of SGPIO_134"]
pub type OutputValueOfSgpio134R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO135` reader - Output value of SGPIO_135"]
pub type OutputValueOfSgpio135R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO136` reader - Output value of SGPIO_136"]
pub type OutputValueOfSgpio136R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO137` reader - Output value of SGPIO_137"]
pub type OutputValueOfSgpio137R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO138` reader - Output value of SGPIO_138"]
pub type OutputValueOfSgpio138R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO139` reader - Output value of SGPIO_139"]
pub type OutputValueOfSgpio139R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO140` reader - Output value of SGPIO_140"]
pub type OutputValueOfSgpio140R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO141` reader - Output value of SGPIO_141"]
pub type OutputValueOfSgpio141R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO142` reader - Output value of SGPIO_142"]
pub type OutputValueOfSgpio142R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO143` reader - Output value of SGPIO_143"]
pub type OutputValueOfSgpio143R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO144` reader - Output value of SGPIO_144"]
pub type OutputValueOfSgpio144R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO145` reader - Output value of SGPIO_145"]
pub type OutputValueOfSgpio145R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO146` reader - Output value of SGPIO_146"]
pub type OutputValueOfSgpio146R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO147` reader - Output value of SGPIO_147"]
pub type OutputValueOfSgpio147R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO148` reader - Output value of SGPIO_148"]
pub type OutputValueOfSgpio148R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO149` reader - Output value of SGPIO_149"]
pub type OutputValueOfSgpio149R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO150` reader - Output value of SGPIO_150"]
pub type OutputValueOfSgpio150R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO151` reader - Output value of SGPIO_151"]
pub type OutputValueOfSgpio151R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO152` reader - Output value of SGPIO_152"]
pub type OutputValueOfSgpio152R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO153` reader - Output value of SGPIO_153"]
pub type OutputValueOfSgpio153R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO154` reader - Output value of SGPIO_154"]
pub type OutputValueOfSgpio154R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO155` reader - Output value of SGPIO_155"]
pub type OutputValueOfSgpio155R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO156` reader - Output value of SGPIO_156"]
pub type OutputValueOfSgpio156R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO157` reader - Output value of SGPIO_157"]
pub type OutputValueOfSgpio157R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO158` reader - Output value of SGPIO_158"]
pub type OutputValueOfSgpio158R = crate::BitReader;
#[doc = "Field `OutputValueOfSGPIO159` reader - Output value of SGPIO_159"]
pub type OutputValueOfSgpio159R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Output value of SGPIO_128"]
    #[inline(always)]
    pub fn output_value_of_sgpio128(&self) -> OutputValueOfSgpio128R {
        OutputValueOfSgpio128R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output value of SGPIO_129"]
    #[inline(always)]
    pub fn output_value_of_sgpio129(&self) -> OutputValueOfSgpio129R {
        OutputValueOfSgpio129R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output value of SGPIO_130"]
    #[inline(always)]
    pub fn output_value_of_sgpio130(&self) -> OutputValueOfSgpio130R {
        OutputValueOfSgpio130R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output value of SGPIO_131"]
    #[inline(always)]
    pub fn output_value_of_sgpio131(&self) -> OutputValueOfSgpio131R {
        OutputValueOfSgpio131R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output value of SGPIO_132"]
    #[inline(always)]
    pub fn output_value_of_sgpio132(&self) -> OutputValueOfSgpio132R {
        OutputValueOfSgpio132R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Output value of SGPIO_133"]
    #[inline(always)]
    pub fn output_value_of_sgpio133(&self) -> OutputValueOfSgpio133R {
        OutputValueOfSgpio133R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Output value of SGPIO_134"]
    #[inline(always)]
    pub fn output_value_of_sgpio134(&self) -> OutputValueOfSgpio134R {
        OutputValueOfSgpio134R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output value of SGPIO_135"]
    #[inline(always)]
    pub fn output_value_of_sgpio135(&self) -> OutputValueOfSgpio135R {
        OutputValueOfSgpio135R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output value of SGPIO_136"]
    #[inline(always)]
    pub fn output_value_of_sgpio136(&self) -> OutputValueOfSgpio136R {
        OutputValueOfSgpio136R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output value of SGPIO_137"]
    #[inline(always)]
    pub fn output_value_of_sgpio137(&self) -> OutputValueOfSgpio137R {
        OutputValueOfSgpio137R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output value of SGPIO_138"]
    #[inline(always)]
    pub fn output_value_of_sgpio138(&self) -> OutputValueOfSgpio138R {
        OutputValueOfSgpio138R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output value of SGPIO_139"]
    #[inline(always)]
    pub fn output_value_of_sgpio139(&self) -> OutputValueOfSgpio139R {
        OutputValueOfSgpio139R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output value of SGPIO_140"]
    #[inline(always)]
    pub fn output_value_of_sgpio140(&self) -> OutputValueOfSgpio140R {
        OutputValueOfSgpio140R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output value of SGPIO_141"]
    #[inline(always)]
    pub fn output_value_of_sgpio141(&self) -> OutputValueOfSgpio141R {
        OutputValueOfSgpio141R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output value of SGPIO_142"]
    #[inline(always)]
    pub fn output_value_of_sgpio142(&self) -> OutputValueOfSgpio142R {
        OutputValueOfSgpio142R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Output value of SGPIO_143"]
    #[inline(always)]
    pub fn output_value_of_sgpio143(&self) -> OutputValueOfSgpio143R {
        OutputValueOfSgpio143R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Output value of SGPIO_144"]
    #[inline(always)]
    pub fn output_value_of_sgpio144(&self) -> OutputValueOfSgpio144R {
        OutputValueOfSgpio144R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Output value of SGPIO_145"]
    #[inline(always)]
    pub fn output_value_of_sgpio145(&self) -> OutputValueOfSgpio145R {
        OutputValueOfSgpio145R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output value of SGPIO_146"]
    #[inline(always)]
    pub fn output_value_of_sgpio146(&self) -> OutputValueOfSgpio146R {
        OutputValueOfSgpio146R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output value of SGPIO_147"]
    #[inline(always)]
    pub fn output_value_of_sgpio147(&self) -> OutputValueOfSgpio147R {
        OutputValueOfSgpio147R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output value of SGPIO_148"]
    #[inline(always)]
    pub fn output_value_of_sgpio148(&self) -> OutputValueOfSgpio148R {
        OutputValueOfSgpio148R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Output value of SGPIO_149"]
    #[inline(always)]
    pub fn output_value_of_sgpio149(&self) -> OutputValueOfSgpio149R {
        OutputValueOfSgpio149R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Output value of SGPIO_150"]
    #[inline(always)]
    pub fn output_value_of_sgpio150(&self) -> OutputValueOfSgpio150R {
        OutputValueOfSgpio150R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Output value of SGPIO_151"]
    #[inline(always)]
    pub fn output_value_of_sgpio151(&self) -> OutputValueOfSgpio151R {
        OutputValueOfSgpio151R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Output value of SGPIO_152"]
    #[inline(always)]
    pub fn output_value_of_sgpio152(&self) -> OutputValueOfSgpio152R {
        OutputValueOfSgpio152R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Output value of SGPIO_153"]
    #[inline(always)]
    pub fn output_value_of_sgpio153(&self) -> OutputValueOfSgpio153R {
        OutputValueOfSgpio153R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Output value of SGPIO_154"]
    #[inline(always)]
    pub fn output_value_of_sgpio154(&self) -> OutputValueOfSgpio154R {
        OutputValueOfSgpio154R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Output value of SGPIO_155"]
    #[inline(always)]
    pub fn output_value_of_sgpio155(&self) -> OutputValueOfSgpio155R {
        OutputValueOfSgpio155R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Output value of SGPIO_156"]
    #[inline(always)]
    pub fn output_value_of_sgpio156(&self) -> OutputValueOfSgpio156R {
        OutputValueOfSgpio156R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Output value of SGPIO_157"]
    #[inline(always)]
    pub fn output_value_of_sgpio157(&self) -> OutputValueOfSgpio157R {
        OutputValueOfSgpio157R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Output value of SGPIO_158"]
    #[inline(always)]
    pub fn output_value_of_sgpio158(&self) -> OutputValueOfSgpio158R {
        OutputValueOfSgpio158R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Output value of SGPIO_159"]
    #[inline(always)]
    pub fn output_value_of_sgpio159(&self) -> OutputValueOfSgpio159R {
        OutputValueOfSgpio159R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Debug Serial Out Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio034::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio034::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio034Spec;
impl crate::RegisterSpec for Sgpio034Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio034::R`](R) reader structure"]
impl crate::Readable for Sgpio034Spec {}
#[doc = "`write(|w| ..)` method takes [`sgpio034::W`](W) writer structure"]
impl crate::Writable for Sgpio034Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO034 to value 0"]
impl crate::Resettable for Sgpio034Spec {}

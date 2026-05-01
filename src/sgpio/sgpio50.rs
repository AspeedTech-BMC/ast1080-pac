#[doc = "Register `SGPIO50` reader"]
pub type R = crate::R<Sgpio50Spec>;
#[doc = "Register `SGPIO50` writer"]
pub type W = crate::W<Sgpio50Spec>;
#[doc = "Field `INTStatusOfSGPIO128` reader - Interrupt Status of SGPIO_128"]
pub type IntstatusOfSgpio128R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO129` reader - Interrupt Status of SGPIO_129"]
pub type IntstatusOfSgpio129R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO130` reader - Interrupt Status of SGPIO_130"]
pub type IntstatusOfSgpio130R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO131` reader - Interrupt Status of SGPIO_131"]
pub type IntstatusOfSgpio131R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO132` reader - Interrupt Status of SGPIO_132"]
pub type IntstatusOfSgpio132R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO133` reader - Interrupt Status of SGPIO_133"]
pub type IntstatusOfSgpio133R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO134` reader - Interrupt Status of SGPIO_134"]
pub type IntstatusOfSgpio134R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO135` reader - Interrupt Status of SGPIO_135"]
pub type IntstatusOfSgpio135R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO136` reader - Interrupt Status of SGPIO_136"]
pub type IntstatusOfSgpio136R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO137` reader - Interrupt Status of SGPIO_137"]
pub type IntstatusOfSgpio137R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO138` reader - Interrupt Status of SGPIO_138"]
pub type IntstatusOfSgpio138R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO139` reader - Interrupt Status of SGPIO_139"]
pub type IntstatusOfSgpio139R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO140` reader - Interrupt Status of SGPIO_140"]
pub type IntstatusOfSgpio140R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO141` reader - Interrupt Status of SGPIO_141"]
pub type IntstatusOfSgpio141R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO142` reader - Interrupt Status of SGPIO_142"]
pub type IntstatusOfSgpio142R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO143` reader - Interrupt Status of SGPIO_143"]
pub type IntstatusOfSgpio143R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO144` reader - Interrupt Status of SGPIO_144"]
pub type IntstatusOfSgpio144R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO145` reader - Interrupt Status of SGPIO_145"]
pub type IntstatusOfSgpio145R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO146` reader - Interrupt Status of SGPIO_146"]
pub type IntstatusOfSgpio146R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO147` reader - Interrupt Status of SGPIO_147"]
pub type IntstatusOfSgpio147R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO148` reader - Interrupt Status of SGPIO_148"]
pub type IntstatusOfSgpio148R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO149` reader - Interrupt Status of SGPIO_149"]
pub type IntstatusOfSgpio149R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO150` reader - Interrupt Status of SGPIO_150"]
pub type IntstatusOfSgpio150R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO151` reader - Interrupt Status of SGPIO_151"]
pub type IntstatusOfSgpio151R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO152` reader - Interrupt Status of SGPIO_152"]
pub type IntstatusOfSgpio152R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO153` reader - Interrupt Status of SGPIO_153"]
pub type IntstatusOfSgpio153R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO154` reader - Interrupt Status of SGPIO_154"]
pub type IntstatusOfSgpio154R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO155` reader - Interrupt Status of SGPIO_155"]
pub type IntstatusOfSgpio155R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO156` reader - Interrupt Status of SGPIO_156"]
pub type IntstatusOfSgpio156R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO157` reader - Interrupt Status of SGPIO_157"]
pub type IntstatusOfSgpio157R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO158` reader - Interrupt Status of SGPIO_158"]
pub type IntstatusOfSgpio158R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO159` reader - Interrupt Status of SGPIO_159"]
pub type IntstatusOfSgpio159R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status of SGPIO_128"]
    #[inline(always)]
    pub fn intstatus_of_sgpio128(&self) -> IntstatusOfSgpio128R {
        IntstatusOfSgpio128R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status of SGPIO_129"]
    #[inline(always)]
    pub fn intstatus_of_sgpio129(&self) -> IntstatusOfSgpio129R {
        IntstatusOfSgpio129R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status of SGPIO_130"]
    #[inline(always)]
    pub fn intstatus_of_sgpio130(&self) -> IntstatusOfSgpio130R {
        IntstatusOfSgpio130R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status of SGPIO_131"]
    #[inline(always)]
    pub fn intstatus_of_sgpio131(&self) -> IntstatusOfSgpio131R {
        IntstatusOfSgpio131R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status of SGPIO_132"]
    #[inline(always)]
    pub fn intstatus_of_sgpio132(&self) -> IntstatusOfSgpio132R {
        IntstatusOfSgpio132R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status of SGPIO_133"]
    #[inline(always)]
    pub fn intstatus_of_sgpio133(&self) -> IntstatusOfSgpio133R {
        IntstatusOfSgpio133R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status of SGPIO_134"]
    #[inline(always)]
    pub fn intstatus_of_sgpio134(&self) -> IntstatusOfSgpio134R {
        IntstatusOfSgpio134R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status of SGPIO_135"]
    #[inline(always)]
    pub fn intstatus_of_sgpio135(&self) -> IntstatusOfSgpio135R {
        IntstatusOfSgpio135R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status of SGPIO_136"]
    #[inline(always)]
    pub fn intstatus_of_sgpio136(&self) -> IntstatusOfSgpio136R {
        IntstatusOfSgpio136R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status of SGPIO_137"]
    #[inline(always)]
    pub fn intstatus_of_sgpio137(&self) -> IntstatusOfSgpio137R {
        IntstatusOfSgpio137R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status of SGPIO_138"]
    #[inline(always)]
    pub fn intstatus_of_sgpio138(&self) -> IntstatusOfSgpio138R {
        IntstatusOfSgpio138R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status of SGPIO_139"]
    #[inline(always)]
    pub fn intstatus_of_sgpio139(&self) -> IntstatusOfSgpio139R {
        IntstatusOfSgpio139R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status of SGPIO_140"]
    #[inline(always)]
    pub fn intstatus_of_sgpio140(&self) -> IntstatusOfSgpio140R {
        IntstatusOfSgpio140R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status of SGPIO_141"]
    #[inline(always)]
    pub fn intstatus_of_sgpio141(&self) -> IntstatusOfSgpio141R {
        IntstatusOfSgpio141R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status of SGPIO_142"]
    #[inline(always)]
    pub fn intstatus_of_sgpio142(&self) -> IntstatusOfSgpio142R {
        IntstatusOfSgpio142R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status of SGPIO_143"]
    #[inline(always)]
    pub fn intstatus_of_sgpio143(&self) -> IntstatusOfSgpio143R {
        IntstatusOfSgpio143R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Status of SGPIO_144"]
    #[inline(always)]
    pub fn intstatus_of_sgpio144(&self) -> IntstatusOfSgpio144R {
        IntstatusOfSgpio144R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Status of SGPIO_145"]
    #[inline(always)]
    pub fn intstatus_of_sgpio145(&self) -> IntstatusOfSgpio145R {
        IntstatusOfSgpio145R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Status of SGPIO_146"]
    #[inline(always)]
    pub fn intstatus_of_sgpio146(&self) -> IntstatusOfSgpio146R {
        IntstatusOfSgpio146R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Status of SGPIO_147"]
    #[inline(always)]
    pub fn intstatus_of_sgpio147(&self) -> IntstatusOfSgpio147R {
        IntstatusOfSgpio147R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Status of SGPIO_148"]
    #[inline(always)]
    pub fn intstatus_of_sgpio148(&self) -> IntstatusOfSgpio148R {
        IntstatusOfSgpio148R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Status of SGPIO_149"]
    #[inline(always)]
    pub fn intstatus_of_sgpio149(&self) -> IntstatusOfSgpio149R {
        IntstatusOfSgpio149R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Status of SGPIO_150"]
    #[inline(always)]
    pub fn intstatus_of_sgpio150(&self) -> IntstatusOfSgpio150R {
        IntstatusOfSgpio150R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Status of SGPIO_151"]
    #[inline(always)]
    pub fn intstatus_of_sgpio151(&self) -> IntstatusOfSgpio151R {
        IntstatusOfSgpio151R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Status of SGPIO_152"]
    #[inline(always)]
    pub fn intstatus_of_sgpio152(&self) -> IntstatusOfSgpio152R {
        IntstatusOfSgpio152R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Status of SGPIO_153"]
    #[inline(always)]
    pub fn intstatus_of_sgpio153(&self) -> IntstatusOfSgpio153R {
        IntstatusOfSgpio153R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Status of SGPIO_154"]
    #[inline(always)]
    pub fn intstatus_of_sgpio154(&self) -> IntstatusOfSgpio154R {
        IntstatusOfSgpio154R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Status of SGPIO_155"]
    #[inline(always)]
    pub fn intstatus_of_sgpio155(&self) -> IntstatusOfSgpio155R {
        IntstatusOfSgpio155R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Status of SGPIO_156"]
    #[inline(always)]
    pub fn intstatus_of_sgpio156(&self) -> IntstatusOfSgpio156R {
        IntstatusOfSgpio156R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Status of SGPIO_157"]
    #[inline(always)]
    pub fn intstatus_of_sgpio157(&self) -> IntstatusOfSgpio157R {
        IntstatusOfSgpio157R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Status of SGPIO_158"]
    #[inline(always)]
    pub fn intstatus_of_sgpio158(&self) -> IntstatusOfSgpio158R {
        IntstatusOfSgpio158R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Status of SGPIO_159"]
    #[inline(always)]
    pub fn intstatus_of_sgpio159(&self) -> IntstatusOfSgpio159R {
        IntstatusOfSgpio159R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt Status Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio50Spec;
impl crate::RegisterSpec for Sgpio50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio50::R`](R) reader structure"]
impl crate::Readable for Sgpio50Spec {}
#[doc = "`write(|w| ..)` method takes [`sgpio50::W`](W) writer structure"]
impl crate::Writable for Sgpio50Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO50 to value 0"]
impl crate::Resettable for Sgpio50Spec {}

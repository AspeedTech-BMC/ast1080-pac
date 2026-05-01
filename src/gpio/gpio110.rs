#[doc = "Register `GPIO110` reader"]
pub type R = crate::R<Gpio110Spec>;
#[doc = "Register `GPIO110` writer"]
pub type W = crate::W<Gpio110Spec>;
#[doc = "Field `INTStatusOfGPIO128` reader - Interrupt Status of GPIO128"]
pub type IntstatusOfGpio128R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO129` reader - Interrupt Status of GPIO129"]
pub type IntstatusOfGpio129R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO130` reader - Interrupt Status of GPIO130"]
pub type IntstatusOfGpio130R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO131` reader - Interrupt Status of GPIO131"]
pub type IntstatusOfGpio131R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO132` reader - Interrupt Status of GPIO132"]
pub type IntstatusOfGpio132R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO133` reader - Interrupt Status of GPIO133"]
pub type IntstatusOfGpio133R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO134` reader - Interrupt Status of GPIO134"]
pub type IntstatusOfGpio134R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO135` reader - Interrupt Status of GPIO135"]
pub type IntstatusOfGpio135R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO136` reader - Interrupt Status of GPIO136"]
pub type IntstatusOfGpio136R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO137` reader - Interrupt Status of GPIO137"]
pub type IntstatusOfGpio137R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO138` reader - Interrupt Status of GPIO138"]
pub type IntstatusOfGpio138R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO139` reader - Interrupt Status of GPIO139"]
pub type IntstatusOfGpio139R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO140` reader - Interrupt Status of GPIO140"]
pub type IntstatusOfGpio140R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO141` reader - Interrupt Status of GPIO141"]
pub type IntstatusOfGpio141R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO142` reader - Interrupt Status of GPIO142"]
pub type IntstatusOfGpio142R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO143` reader - Interrupt Status of GPIO143"]
pub type IntstatusOfGpio143R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO144` reader - Interrupt Status of GPIO144"]
pub type IntstatusOfGpio144R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO145` reader - Interrupt Status of GPIO145"]
pub type IntstatusOfGpio145R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO146` reader - Interrupt Status of GPIO146"]
pub type IntstatusOfGpio146R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO147` reader - Interrupt Status of GPIO147"]
pub type IntstatusOfGpio147R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO148` reader - Interrupt Status of GPIO148"]
pub type IntstatusOfGpio148R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO149` reader - Interrupt Status of GPIO149"]
pub type IntstatusOfGpio149R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO150` reader - Interrupt Status of GPIO150"]
pub type IntstatusOfGpio150R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO151` reader - Interrupt Status of GPIO151"]
pub type IntstatusOfGpio151R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO152` reader - Interrupt Status of GPIO152"]
pub type IntstatusOfGpio152R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO153` reader - Interrupt Status of GPIO153"]
pub type IntstatusOfGpio153R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO154` reader - Interrupt Status of GPIO154"]
pub type IntstatusOfGpio154R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO155` reader - Interrupt Status of GPIO155"]
pub type IntstatusOfGpio155R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO156` reader - Interrupt Status of GPIO156"]
pub type IntstatusOfGpio156R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO157` reader - Interrupt Status of GPIO157"]
pub type IntstatusOfGpio157R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO158` reader - Interrupt Status of GPIO158"]
pub type IntstatusOfGpio158R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO159` reader - Interrupt Status of GPIO159"]
pub type IntstatusOfGpio159R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status of GPIO128"]
    #[inline(always)]
    pub fn intstatus_of_gpio128(&self) -> IntstatusOfGpio128R {
        IntstatusOfGpio128R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status of GPIO129"]
    #[inline(always)]
    pub fn intstatus_of_gpio129(&self) -> IntstatusOfGpio129R {
        IntstatusOfGpio129R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status of GPIO130"]
    #[inline(always)]
    pub fn intstatus_of_gpio130(&self) -> IntstatusOfGpio130R {
        IntstatusOfGpio130R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status of GPIO131"]
    #[inline(always)]
    pub fn intstatus_of_gpio131(&self) -> IntstatusOfGpio131R {
        IntstatusOfGpio131R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status of GPIO132"]
    #[inline(always)]
    pub fn intstatus_of_gpio132(&self) -> IntstatusOfGpio132R {
        IntstatusOfGpio132R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status of GPIO133"]
    #[inline(always)]
    pub fn intstatus_of_gpio133(&self) -> IntstatusOfGpio133R {
        IntstatusOfGpio133R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status of GPIO134"]
    #[inline(always)]
    pub fn intstatus_of_gpio134(&self) -> IntstatusOfGpio134R {
        IntstatusOfGpio134R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status of GPIO135"]
    #[inline(always)]
    pub fn intstatus_of_gpio135(&self) -> IntstatusOfGpio135R {
        IntstatusOfGpio135R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status of GPIO136"]
    #[inline(always)]
    pub fn intstatus_of_gpio136(&self) -> IntstatusOfGpio136R {
        IntstatusOfGpio136R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status of GPIO137"]
    #[inline(always)]
    pub fn intstatus_of_gpio137(&self) -> IntstatusOfGpio137R {
        IntstatusOfGpio137R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status of GPIO138"]
    #[inline(always)]
    pub fn intstatus_of_gpio138(&self) -> IntstatusOfGpio138R {
        IntstatusOfGpio138R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status of GPIO139"]
    #[inline(always)]
    pub fn intstatus_of_gpio139(&self) -> IntstatusOfGpio139R {
        IntstatusOfGpio139R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status of GPIO140"]
    #[inline(always)]
    pub fn intstatus_of_gpio140(&self) -> IntstatusOfGpio140R {
        IntstatusOfGpio140R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status of GPIO141"]
    #[inline(always)]
    pub fn intstatus_of_gpio141(&self) -> IntstatusOfGpio141R {
        IntstatusOfGpio141R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status of GPIO142"]
    #[inline(always)]
    pub fn intstatus_of_gpio142(&self) -> IntstatusOfGpio142R {
        IntstatusOfGpio142R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status of GPIO143"]
    #[inline(always)]
    pub fn intstatus_of_gpio143(&self) -> IntstatusOfGpio143R {
        IntstatusOfGpio143R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Status of GPIO144"]
    #[inline(always)]
    pub fn intstatus_of_gpio144(&self) -> IntstatusOfGpio144R {
        IntstatusOfGpio144R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Status of GPIO145"]
    #[inline(always)]
    pub fn intstatus_of_gpio145(&self) -> IntstatusOfGpio145R {
        IntstatusOfGpio145R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Status of GPIO146"]
    #[inline(always)]
    pub fn intstatus_of_gpio146(&self) -> IntstatusOfGpio146R {
        IntstatusOfGpio146R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Status of GPIO147"]
    #[inline(always)]
    pub fn intstatus_of_gpio147(&self) -> IntstatusOfGpio147R {
        IntstatusOfGpio147R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Status of GPIO148"]
    #[inline(always)]
    pub fn intstatus_of_gpio148(&self) -> IntstatusOfGpio148R {
        IntstatusOfGpio148R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Status of GPIO149"]
    #[inline(always)]
    pub fn intstatus_of_gpio149(&self) -> IntstatusOfGpio149R {
        IntstatusOfGpio149R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Status of GPIO150"]
    #[inline(always)]
    pub fn intstatus_of_gpio150(&self) -> IntstatusOfGpio150R {
        IntstatusOfGpio150R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Status of GPIO151"]
    #[inline(always)]
    pub fn intstatus_of_gpio151(&self) -> IntstatusOfGpio151R {
        IntstatusOfGpio151R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Status of GPIO152"]
    #[inline(always)]
    pub fn intstatus_of_gpio152(&self) -> IntstatusOfGpio152R {
        IntstatusOfGpio152R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Status of GPIO153"]
    #[inline(always)]
    pub fn intstatus_of_gpio153(&self) -> IntstatusOfGpio153R {
        IntstatusOfGpio153R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Status of GPIO154"]
    #[inline(always)]
    pub fn intstatus_of_gpio154(&self) -> IntstatusOfGpio154R {
        IntstatusOfGpio154R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Status of GPIO155"]
    #[inline(always)]
    pub fn intstatus_of_gpio155(&self) -> IntstatusOfGpio155R {
        IntstatusOfGpio155R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Status of GPIO156"]
    #[inline(always)]
    pub fn intstatus_of_gpio156(&self) -> IntstatusOfGpio156R {
        IntstatusOfGpio156R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Status of GPIO157"]
    #[inline(always)]
    pub fn intstatus_of_gpio157(&self) -> IntstatusOfGpio157R {
        IntstatusOfGpio157R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Status of GPIO158"]
    #[inline(always)]
    pub fn intstatus_of_gpio158(&self) -> IntstatusOfGpio158R {
        IntstatusOfGpio158R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Status of GPIO159"]
    #[inline(always)]
    pub fn intstatus_of_gpio159(&self) -> IntstatusOfGpio159R {
        IntstatusOfGpio159R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt Status Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio110::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio110::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio110Spec;
impl crate::RegisterSpec for Gpio110Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio110::R`](R) reader structure"]
impl crate::Readable for Gpio110Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio110::W`](W) writer structure"]
impl crate::Writable for Gpio110Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO110 to value 0"]
impl crate::Resettable for Gpio110Spec {}

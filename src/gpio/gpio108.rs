#[doc = "Register `GPIO108` reader"]
pub type R = crate::R<Gpio108Spec>;
#[doc = "Register `GPIO108` writer"]
pub type W = crate::W<Gpio108Spec>;
#[doc = "Field `INTStatusOfGPIO064` reader - Interrupt Status of GPIO064"]
pub type IntstatusOfGpio064R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO065` reader - Interrupt Status of GPIO065"]
pub type IntstatusOfGpio065R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO066` reader - Interrupt Status of GPIO066"]
pub type IntstatusOfGpio066R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO067` reader - Interrupt Status of GPIO067"]
pub type IntstatusOfGpio067R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO068` reader - Interrupt Status of GPIO068"]
pub type IntstatusOfGpio068R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO069` reader - Interrupt Status of GPIO069"]
pub type IntstatusOfGpio069R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO070` reader - Interrupt Status of GPIO070"]
pub type IntstatusOfGpio070R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO071` reader - Interrupt Status of GPIO071"]
pub type IntstatusOfGpio071R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO072` reader - Interrupt Status of GPIO072"]
pub type IntstatusOfGpio072R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO073` reader - Interrupt Status of GPIO073"]
pub type IntstatusOfGpio073R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO074` reader - Interrupt Status of GPIO074"]
pub type IntstatusOfGpio074R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO075` reader - Interrupt Status of GPIO075"]
pub type IntstatusOfGpio075R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO076` reader - Interrupt Status of GPIO076"]
pub type IntstatusOfGpio076R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO077` reader - Interrupt Status of GPIO077"]
pub type IntstatusOfGpio077R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO078` reader - Interrupt Status of GPIO078"]
pub type IntstatusOfGpio078R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO079` reader - Interrupt Status of GPIO079"]
pub type IntstatusOfGpio079R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO080` reader - Interrupt Status of GPIO080"]
pub type IntstatusOfGpio080R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO081` reader - Interrupt Status of GPIO081"]
pub type IntstatusOfGpio081R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO082` reader - Interrupt Status of GPIO082"]
pub type IntstatusOfGpio082R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO083` reader - Interrupt Status of GPIO083"]
pub type IntstatusOfGpio083R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO084` reader - Interrupt Status of GPIO084"]
pub type IntstatusOfGpio084R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO085` reader - Interrupt Status of GPIO085"]
pub type IntstatusOfGpio085R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO086` reader - Interrupt Status of GPIO086"]
pub type IntstatusOfGpio086R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO087` reader - Interrupt Status of GPIO087"]
pub type IntstatusOfGpio087R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO088` reader - Interrupt Status of GPIO088"]
pub type IntstatusOfGpio088R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO089` reader - Interrupt Status of GPIO089"]
pub type IntstatusOfGpio089R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO090` reader - Interrupt Status of GPIO090"]
pub type IntstatusOfGpio090R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO091` reader - Interrupt Status of GPIO091"]
pub type IntstatusOfGpio091R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO092` reader - Interrupt Status of GPIO092"]
pub type IntstatusOfGpio092R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO093` reader - Interrupt Status of GPIO093"]
pub type IntstatusOfGpio093R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO094` reader - Interrupt Status of GPIO094"]
pub type IntstatusOfGpio094R = crate::BitReader;
#[doc = "Field `INTStatusOfGPIO095` reader - Interrupt Status of GPIO095"]
pub type IntstatusOfGpio095R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status of GPIO064"]
    #[inline(always)]
    pub fn intstatus_of_gpio064(&self) -> IntstatusOfGpio064R {
        IntstatusOfGpio064R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status of GPIO065"]
    #[inline(always)]
    pub fn intstatus_of_gpio065(&self) -> IntstatusOfGpio065R {
        IntstatusOfGpio065R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status of GPIO066"]
    #[inline(always)]
    pub fn intstatus_of_gpio066(&self) -> IntstatusOfGpio066R {
        IntstatusOfGpio066R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status of GPIO067"]
    #[inline(always)]
    pub fn intstatus_of_gpio067(&self) -> IntstatusOfGpio067R {
        IntstatusOfGpio067R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status of GPIO068"]
    #[inline(always)]
    pub fn intstatus_of_gpio068(&self) -> IntstatusOfGpio068R {
        IntstatusOfGpio068R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status of GPIO069"]
    #[inline(always)]
    pub fn intstatus_of_gpio069(&self) -> IntstatusOfGpio069R {
        IntstatusOfGpio069R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status of GPIO070"]
    #[inline(always)]
    pub fn intstatus_of_gpio070(&self) -> IntstatusOfGpio070R {
        IntstatusOfGpio070R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status of GPIO071"]
    #[inline(always)]
    pub fn intstatus_of_gpio071(&self) -> IntstatusOfGpio071R {
        IntstatusOfGpio071R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status of GPIO072"]
    #[inline(always)]
    pub fn intstatus_of_gpio072(&self) -> IntstatusOfGpio072R {
        IntstatusOfGpio072R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status of GPIO073"]
    #[inline(always)]
    pub fn intstatus_of_gpio073(&self) -> IntstatusOfGpio073R {
        IntstatusOfGpio073R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status of GPIO074"]
    #[inline(always)]
    pub fn intstatus_of_gpio074(&self) -> IntstatusOfGpio074R {
        IntstatusOfGpio074R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status of GPIO075"]
    #[inline(always)]
    pub fn intstatus_of_gpio075(&self) -> IntstatusOfGpio075R {
        IntstatusOfGpio075R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status of GPIO076"]
    #[inline(always)]
    pub fn intstatus_of_gpio076(&self) -> IntstatusOfGpio076R {
        IntstatusOfGpio076R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status of GPIO077"]
    #[inline(always)]
    pub fn intstatus_of_gpio077(&self) -> IntstatusOfGpio077R {
        IntstatusOfGpio077R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status of GPIO078"]
    #[inline(always)]
    pub fn intstatus_of_gpio078(&self) -> IntstatusOfGpio078R {
        IntstatusOfGpio078R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status of GPIO079"]
    #[inline(always)]
    pub fn intstatus_of_gpio079(&self) -> IntstatusOfGpio079R {
        IntstatusOfGpio079R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Status of GPIO080"]
    #[inline(always)]
    pub fn intstatus_of_gpio080(&self) -> IntstatusOfGpio080R {
        IntstatusOfGpio080R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Status of GPIO081"]
    #[inline(always)]
    pub fn intstatus_of_gpio081(&self) -> IntstatusOfGpio081R {
        IntstatusOfGpio081R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Status of GPIO082"]
    #[inline(always)]
    pub fn intstatus_of_gpio082(&self) -> IntstatusOfGpio082R {
        IntstatusOfGpio082R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Status of GPIO083"]
    #[inline(always)]
    pub fn intstatus_of_gpio083(&self) -> IntstatusOfGpio083R {
        IntstatusOfGpio083R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Status of GPIO084"]
    #[inline(always)]
    pub fn intstatus_of_gpio084(&self) -> IntstatusOfGpio084R {
        IntstatusOfGpio084R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Status of GPIO085"]
    #[inline(always)]
    pub fn intstatus_of_gpio085(&self) -> IntstatusOfGpio085R {
        IntstatusOfGpio085R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Status of GPIO086"]
    #[inline(always)]
    pub fn intstatus_of_gpio086(&self) -> IntstatusOfGpio086R {
        IntstatusOfGpio086R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Status of GPIO087"]
    #[inline(always)]
    pub fn intstatus_of_gpio087(&self) -> IntstatusOfGpio087R {
        IntstatusOfGpio087R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Status of GPIO088"]
    #[inline(always)]
    pub fn intstatus_of_gpio088(&self) -> IntstatusOfGpio088R {
        IntstatusOfGpio088R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Status of GPIO089"]
    #[inline(always)]
    pub fn intstatus_of_gpio089(&self) -> IntstatusOfGpio089R {
        IntstatusOfGpio089R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Status of GPIO090"]
    #[inline(always)]
    pub fn intstatus_of_gpio090(&self) -> IntstatusOfGpio090R {
        IntstatusOfGpio090R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Status of GPIO091"]
    #[inline(always)]
    pub fn intstatus_of_gpio091(&self) -> IntstatusOfGpio091R {
        IntstatusOfGpio091R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Status of GPIO092"]
    #[inline(always)]
    pub fn intstatus_of_gpio092(&self) -> IntstatusOfGpio092R {
        IntstatusOfGpio092R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Status of GPIO093"]
    #[inline(always)]
    pub fn intstatus_of_gpio093(&self) -> IntstatusOfGpio093R {
        IntstatusOfGpio093R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Status of GPIO094"]
    #[inline(always)]
    pub fn intstatus_of_gpio094(&self) -> IntstatusOfGpio094R {
        IntstatusOfGpio094R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Status of GPIO095"]
    #[inline(always)]
    pub fn intstatus_of_gpio095(&self) -> IntstatusOfGpio095R {
        IntstatusOfGpio095R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt Status Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio108::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio108::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio108Spec;
impl crate::RegisterSpec for Gpio108Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio108::R`](R) reader structure"]
impl crate::Readable for Gpio108Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio108::W`](W) writer structure"]
impl crate::Writable for Gpio108Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO108 to value 0"]
impl crate::Resettable for Gpio108Spec {}

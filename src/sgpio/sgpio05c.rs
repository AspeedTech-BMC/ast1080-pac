#[doc = "Register `SGPIO05C` reader"]
pub type R = crate::R<Sgpio05cSpec>;
#[doc = "Register `SGPIO05C` writer"]
pub type W = crate::W<Sgpio05cSpec>;
#[doc = "Field `INTStatusOfSGPIO224` reader - Interrupt Status of SGPIO_224"]
pub type IntstatusOfSgpio224R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO225` reader - Interrupt Status of SGPIO_225"]
pub type IntstatusOfSgpio225R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO226` reader - Interrupt Status of SGPIO_226"]
pub type IntstatusOfSgpio226R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO227` reader - Interrupt Status of SGPIO_227"]
pub type IntstatusOfSgpio227R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO228` reader - Interrupt Status of SGPIO_228"]
pub type IntstatusOfSgpio228R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO229` reader - Interrupt Status of SGPIO_229"]
pub type IntstatusOfSgpio229R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO230` reader - Interrupt Status of SGPIO_230"]
pub type IntstatusOfSgpio230R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO231` reader - Interrupt Status of SGPIO_231"]
pub type IntstatusOfSgpio231R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO232` reader - Interrupt Status of SGPIO_232"]
pub type IntstatusOfSgpio232R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO233` reader - Interrupt Status of SGPIO_233"]
pub type IntstatusOfSgpio233R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO234` reader - Interrupt Status of SGPIO_234"]
pub type IntstatusOfSgpio234R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO235` reader - Interrupt Status of SGPIO_235"]
pub type IntstatusOfSgpio235R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO236` reader - Interrupt Status of SGPIO_236"]
pub type IntstatusOfSgpio236R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO237` reader - Interrupt Status of SGPIO_237"]
pub type IntstatusOfSgpio237R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO238` reader - Interrupt Status of SGPIO_238"]
pub type IntstatusOfSgpio238R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO239` reader - Interrupt Status of SGPIO_239"]
pub type IntstatusOfSgpio239R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO240` reader - Interrupt Status of SGPIO_240"]
pub type IntstatusOfSgpio240R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO241` reader - Interrupt Status of SGPIO_241"]
pub type IntstatusOfSgpio241R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO242` reader - Interrupt Status of SGPIO_242"]
pub type IntstatusOfSgpio242R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO243` reader - Interrupt Status of SGPIO_243"]
pub type IntstatusOfSgpio243R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO244` reader - Interrupt Status of SGPIO_244"]
pub type IntstatusOfSgpio244R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO245` reader - Interrupt Status of SGPIO_245"]
pub type IntstatusOfSgpio245R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO246` reader - Interrupt Status of SGPIO_246"]
pub type IntstatusOfSgpio246R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO247` reader - Interrupt Status of SGPIO_247"]
pub type IntstatusOfSgpio247R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO248` reader - Interrupt Status of SGPIO_248"]
pub type IntstatusOfSgpio248R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO249` reader - Interrupt Status of SGPIO_249"]
pub type IntstatusOfSgpio249R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO250` reader - Interrupt Status of SGPIO_250"]
pub type IntstatusOfSgpio250R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO251` reader - Interrupt Status of SGPIO_251"]
pub type IntstatusOfSgpio251R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO252` reader - Interrupt Status of SGPIO_252"]
pub type IntstatusOfSgpio252R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO253` reader - Interrupt Status of SGPIO_253"]
pub type IntstatusOfSgpio253R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO254` reader - Interrupt Status of SGPIO_254"]
pub type IntstatusOfSgpio254R = crate::BitReader;
#[doc = "Field `INTStatusOfSGPIO255` reader - Interrupt Status of SGPIO_255"]
pub type IntstatusOfSgpio255R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt Status of SGPIO_224"]
    #[inline(always)]
    pub fn intstatus_of_sgpio224(&self) -> IntstatusOfSgpio224R {
        IntstatusOfSgpio224R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status of SGPIO_225"]
    #[inline(always)]
    pub fn intstatus_of_sgpio225(&self) -> IntstatusOfSgpio225R {
        IntstatusOfSgpio225R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status of SGPIO_226"]
    #[inline(always)]
    pub fn intstatus_of_sgpio226(&self) -> IntstatusOfSgpio226R {
        IntstatusOfSgpio226R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status of SGPIO_227"]
    #[inline(always)]
    pub fn intstatus_of_sgpio227(&self) -> IntstatusOfSgpio227R {
        IntstatusOfSgpio227R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status of SGPIO_228"]
    #[inline(always)]
    pub fn intstatus_of_sgpio228(&self) -> IntstatusOfSgpio228R {
        IntstatusOfSgpio228R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status of SGPIO_229"]
    #[inline(always)]
    pub fn intstatus_of_sgpio229(&self) -> IntstatusOfSgpio229R {
        IntstatusOfSgpio229R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status of SGPIO_230"]
    #[inline(always)]
    pub fn intstatus_of_sgpio230(&self) -> IntstatusOfSgpio230R {
        IntstatusOfSgpio230R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status of SGPIO_231"]
    #[inline(always)]
    pub fn intstatus_of_sgpio231(&self) -> IntstatusOfSgpio231R {
        IntstatusOfSgpio231R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status of SGPIO_232"]
    #[inline(always)]
    pub fn intstatus_of_sgpio232(&self) -> IntstatusOfSgpio232R {
        IntstatusOfSgpio232R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status of SGPIO_233"]
    #[inline(always)]
    pub fn intstatus_of_sgpio233(&self) -> IntstatusOfSgpio233R {
        IntstatusOfSgpio233R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status of SGPIO_234"]
    #[inline(always)]
    pub fn intstatus_of_sgpio234(&self) -> IntstatusOfSgpio234R {
        IntstatusOfSgpio234R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status of SGPIO_235"]
    #[inline(always)]
    pub fn intstatus_of_sgpio235(&self) -> IntstatusOfSgpio235R {
        IntstatusOfSgpio235R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status of SGPIO_236"]
    #[inline(always)]
    pub fn intstatus_of_sgpio236(&self) -> IntstatusOfSgpio236R {
        IntstatusOfSgpio236R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status of SGPIO_237"]
    #[inline(always)]
    pub fn intstatus_of_sgpio237(&self) -> IntstatusOfSgpio237R {
        IntstatusOfSgpio237R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status of SGPIO_238"]
    #[inline(always)]
    pub fn intstatus_of_sgpio238(&self) -> IntstatusOfSgpio238R {
        IntstatusOfSgpio238R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status of SGPIO_239"]
    #[inline(always)]
    pub fn intstatus_of_sgpio239(&self) -> IntstatusOfSgpio239R {
        IntstatusOfSgpio239R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Status of SGPIO_240"]
    #[inline(always)]
    pub fn intstatus_of_sgpio240(&self) -> IntstatusOfSgpio240R {
        IntstatusOfSgpio240R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Status of SGPIO_241"]
    #[inline(always)]
    pub fn intstatus_of_sgpio241(&self) -> IntstatusOfSgpio241R {
        IntstatusOfSgpio241R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Status of SGPIO_242"]
    #[inline(always)]
    pub fn intstatus_of_sgpio242(&self) -> IntstatusOfSgpio242R {
        IntstatusOfSgpio242R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Status of SGPIO_243"]
    #[inline(always)]
    pub fn intstatus_of_sgpio243(&self) -> IntstatusOfSgpio243R {
        IntstatusOfSgpio243R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Status of SGPIO_244"]
    #[inline(always)]
    pub fn intstatus_of_sgpio244(&self) -> IntstatusOfSgpio244R {
        IntstatusOfSgpio244R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Status of SGPIO_245"]
    #[inline(always)]
    pub fn intstatus_of_sgpio245(&self) -> IntstatusOfSgpio245R {
        IntstatusOfSgpio245R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Status of SGPIO_246"]
    #[inline(always)]
    pub fn intstatus_of_sgpio246(&self) -> IntstatusOfSgpio246R {
        IntstatusOfSgpio246R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Status of SGPIO_247"]
    #[inline(always)]
    pub fn intstatus_of_sgpio247(&self) -> IntstatusOfSgpio247R {
        IntstatusOfSgpio247R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Status of SGPIO_248"]
    #[inline(always)]
    pub fn intstatus_of_sgpio248(&self) -> IntstatusOfSgpio248R {
        IntstatusOfSgpio248R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Status of SGPIO_249"]
    #[inline(always)]
    pub fn intstatus_of_sgpio249(&self) -> IntstatusOfSgpio249R {
        IntstatusOfSgpio249R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Status of SGPIO_250"]
    #[inline(always)]
    pub fn intstatus_of_sgpio250(&self) -> IntstatusOfSgpio250R {
        IntstatusOfSgpio250R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Status of SGPIO_251"]
    #[inline(always)]
    pub fn intstatus_of_sgpio251(&self) -> IntstatusOfSgpio251R {
        IntstatusOfSgpio251R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Status of SGPIO_252"]
    #[inline(always)]
    pub fn intstatus_of_sgpio252(&self) -> IntstatusOfSgpio252R {
        IntstatusOfSgpio252R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Status of SGPIO_253"]
    #[inline(always)]
    pub fn intstatus_of_sgpio253(&self) -> IntstatusOfSgpio253R {
        IntstatusOfSgpio253R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Status of SGPIO_254"]
    #[inline(always)]
    pub fn intstatus_of_sgpio254(&self) -> IntstatusOfSgpio254R {
        IntstatusOfSgpio254R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Status of SGPIO_255"]
    #[inline(always)]
    pub fn intstatus_of_sgpio255(&self) -> IntstatusOfSgpio255R {
        IntstatusOfSgpio255R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt Status Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`sgpio05c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sgpio05c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgpio05cSpec;
impl crate::RegisterSpec for Sgpio05cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgpio05c::R`](R) reader structure"]
impl crate::Readable for Sgpio05cSpec {}
#[doc = "`write(|w| ..)` method takes [`sgpio05c::W`](W) writer structure"]
impl crate::Writable for Sgpio05cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SGPIO05C to value 0"]
impl crate::Resettable for Sgpio05cSpec {}

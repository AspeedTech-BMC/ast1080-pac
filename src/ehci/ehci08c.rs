#[doc = "Register `EHCI08C` reader"]
pub type R = crate::R<Ehci08cSpec>;
#[doc = "Register `EHCI08C` writer"]
pub type W = crate::W<Ehci08cSpec>;
#[doc = "Field `HwRevNumber` reader - Hardware revision number"]
pub type HwRevNumberR = crate::FieldReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - Hardware revision number"]
    #[inline(always)]
    pub fn hw_rev_number(&self) -> HwRevNumberR {
        HwRevNumberR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Hardware Revision Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci08c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci08c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ehci08cSpec;
impl crate::RegisterSpec for Ehci08cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehci08c::R`](R) reader structure"]
impl crate::Readable for Ehci08cSpec {}
#[doc = "`write(|w| ..)` method takes [`ehci08c::W`](W) writer structure"]
impl crate::Writable for Ehci08cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCI08C to value 0x02"]
impl crate::Resettable for Ehci08cSpec {
    const RESET_VALUE: u32 = 0x02;
}

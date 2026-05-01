#[doc = "Register `EHCI030` reader"]
pub type R = crate::R<Ehci030Spec>;
#[doc = "Register `EHCI030` writer"]
pub type W = crate::W<Ehci030Spec>;
#[doc = "Field `BaseAddrHi` reader - Base Address Hi"]
pub type BaseAddrHiR = crate::FieldReader;
#[doc = "Field `BaseAddrHi` writer - Base Address Hi"]
pub type BaseAddrHiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - Base Address Hi"]
    #[inline(always)]
    pub fn base_addr_hi(&self) -> BaseAddrHiR {
        BaseAddrHiR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:21 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 2) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - Base Address Hi"]
    #[inline(always)]
    pub fn base_addr_hi(&mut self) -> BaseAddrHiW<Ehci030Spec> {
        BaseAddrHiW::new(self, 0)
    }
}
#[doc = "Control Data Structure Segment Register (CTRLDSSEGMENT)\n\nYou can [`read`](crate::Reg::read) this register and get [`ehci030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehci030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ehci030Spec;
impl crate::RegisterSpec for Ehci030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehci030::R`](R) reader structure"]
impl crate::Readable for Ehci030Spec {}
#[doc = "`write(|w| ..)` method takes [`ehci030::W`](W) writer structure"]
impl crate::Writable for Ehci030Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EHCI030 to value 0"]
impl crate::Resettable for Ehci030Spec {}

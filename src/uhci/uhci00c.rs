#[doc = "Register `UHCI00C` reader"]
pub type R = crate::R<Uhci00cSpec>;
#[doc = "Register `UHCI00C` writer"]
pub type W = crate::W<Uhci00cSpec>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `BaseAddr` reader - Base Address"]
pub type BaseAddrR = crate::FieldReader<u32>;
#[doc = "Field `BaseAddr` writer - Base Address"]
pub type BaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:11 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - Base Address"]
    #[inline(always)]
    pub fn base_addr(&self) -> BaseAddrR {
        BaseAddrR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - Base Address"]
    #[inline(always)]
    pub fn base_addr(&mut self) -> BaseAddrW<Uhci00cSpec> {
        BaseAddrW::new(self, 12)
    }
}
#[doc = "Frame List Based Address Register (FRBASEADD)\n\nYou can [`read`](crate::Reg::read) this register and get [`uhci00c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhci00c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uhci00cSpec;
impl crate::RegisterSpec for Uhci00cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhci00c::R`](R) reader structure"]
impl crate::Readable for Uhci00cSpec {}
#[doc = "`write(|w| ..)` method takes [`uhci00c::W`](W) writer structure"]
impl crate::Writable for Uhci00cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UHCI00C to value 0"]
impl crate::Resettable for Uhci00cSpec {}

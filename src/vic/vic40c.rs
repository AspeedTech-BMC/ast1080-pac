#[doc = "Register `VIC40C` reader"]
pub type R = crate::R<Vic40cSpec>;
#[doc = "Register `VIC40C` writer"]
pub type W = crate::W<Vic40cSpec>;
#[doc = "Field `VICSIRQCSEL33` reader - VIC_SIRQ_CSEL3_3"]
pub type Vicsirqcsel33R = crate::FieldReader<u32>;
#[doc = "Field `VICSIRQCSEL33` writer - VIC_SIRQ_CSEL3_3"]
pub type Vicsirqcsel33W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL3_3"]
    #[inline(always)]
    pub fn vicsirqcsel33(&self) -> Vicsirqcsel33R {
        Vicsirqcsel33R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_SIRQ_CSEL3_3"]
    #[inline(always)]
    pub fn vicsirqcsel33(&mut self) -> Vicsirqcsel33W<Vic40cSpec> {
        Vicsirqcsel33W::new(self, 0)
    }
}
#[doc = "Int Routing Select3 3\n\nYou can [`read`](crate::Reg::read) this register and get [`vic40c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic40c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic40cSpec;
impl crate::RegisterSpec for Vic40cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic40c::R`](R) reader structure"]
impl crate::Readable for Vic40cSpec {}
#[doc = "`write(|w| ..)` method takes [`vic40c::W`](W) writer structure"]
impl crate::Writable for Vic40cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC40C to value 0"]
impl crate::Resettable for Vic40cSpec {}

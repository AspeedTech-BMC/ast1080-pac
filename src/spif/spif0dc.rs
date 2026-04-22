#[doc = "Register `SPIF0DC` reader"]
pub type R = crate::R<Spif0dcSpec>;
#[doc = "Register `SPIF0DC` writer"]
pub type W = crate::W<Spif0dcSpec>;
#[doc = "Field `WTABLE23` reader - WTABLE23"]
pub type Wtable23R = crate::FieldReader<u32>;
#[doc = "Field `WTABLE23` writer - WTABLE23"]
pub type Wtable23W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - WTABLE23"]
    #[inline(always)]
    pub fn wtable23(&self) -> Wtable23R {
        Wtable23R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - WTABLE23"]
    #[inline(always)]
    pub fn wtable23(&mut self) -> Wtable23W<Spif0dcSpec> {
        Wtable23W::new(self, 0)
    }
}
#[doc = "SPIF\\_WTABLE23\n\nYou can [`read`](crate::Reg::read) this register and get [`spif0dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif0dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif0dcSpec;
impl crate::RegisterSpec for Spif0dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif0dc::R`](R) reader structure"]
impl crate::Readable for Spif0dcSpec {}
#[doc = "`write(|w| ..)` method takes [`spif0dc::W`](W) writer structure"]
impl crate::Writable for Spif0dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF0DC to value 0"]
impl crate::Resettable for Spif0dcSpec {}

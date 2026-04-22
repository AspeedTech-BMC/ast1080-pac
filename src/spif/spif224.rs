#[doc = "Register `SPIF224` reader"]
pub type R = crate::R<Spif224Spec>;
#[doc = "Register `SPIF224` writer"]
pub type W = crate::W<Spif224Spec>;
#[doc = "Field `ADDRLBND09` reader - ADDR_LBND09"]
pub type Addrlbnd09R = crate::FieldReader<u16>;
#[doc = "Field `ADDRLBND09` writer - ADDR_LBND09"]
pub type Addrlbnd09W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRUBND09` reader - ADDR_UBND09"]
pub type Addrubnd09R = crate::FieldReader<u16>;
#[doc = "Field `ADDRUBND09` writer - ADDR_UBND09"]
pub type Addrubnd09W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADDR_LBND09"]
    #[inline(always)]
    pub fn addrlbnd09(&self) -> Addrlbnd09R {
        Addrlbnd09R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADDR_UBND09"]
    #[inline(always)]
    pub fn addrubnd09(&self) -> Addrubnd09R {
        Addrubnd09R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDR_LBND09"]
    #[inline(always)]
    pub fn addrlbnd09(&mut self) -> Addrlbnd09W<Spif224Spec> {
        Addrlbnd09W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ADDR_UBND09"]
    #[inline(always)]
    pub fn addrubnd09(&mut self) -> Addrubnd09W<Spif224Spec> {
        Addrubnd09W::new(self, 16)
    }
}
#[doc = "SPIF\\_ADDRBND09\n\nYou can [`read`](crate::Reg::read) this register and get [`spif224::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif224::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif224Spec;
impl crate::RegisterSpec for Spif224Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif224::R`](R) reader structure"]
impl crate::Readable for Spif224Spec {}
#[doc = "`write(|w| ..)` method takes [`spif224::W`](W) writer structure"]
impl crate::Writable for Spif224Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF224 to value 0"]
impl crate::Resettable for Spif224Spec {}

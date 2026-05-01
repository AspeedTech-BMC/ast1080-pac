#[doc = "Register `SCU410` reader"]
pub type R = crate::R<Scu410Spec>;
#[doc = "Register `SCU410` writer"]
pub type W = crate::W<Scu410Spec>;
#[doc = "Field `SCUMUXIO032` reader - SCU_MUX_IO032"]
pub type Scumuxio032R = crate::FieldReader;
#[doc = "Field `SCUMUXIO032` writer - SCU_MUX_IO032"]
pub type Scumuxio032W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `SCUMUXIO033` reader - SCU_MUX_IO033"]
pub type Scumuxio033R = crate::FieldReader;
#[doc = "Field `SCUMUXIO033` writer - SCU_MUX_IO033"]
pub type Scumuxio033W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `SCUMUXIO034` reader - SCU_MUX_IO034"]
pub type Scumuxio034R = crate::FieldReader;
#[doc = "Field `SCUMUXIO034` writer - SCU_MUX_IO034"]
pub type Scumuxio034W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUMUXIO035` reader - SCU_MUX_IO035"]
pub type Scumuxio035R = crate::FieldReader;
#[doc = "Field `SCUMUXIO035` writer - SCU_MUX_IO035"]
pub type Scumuxio035W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUMUXIO036` reader - SCU_MUX_IO036"]
pub type Scumuxio036R = crate::FieldReader;
#[doc = "Field `SCUMUXIO036` writer - SCU_MUX_IO036"]
pub type Scumuxio036W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUMUXIO037` reader - SCU_MUX_IO037"]
pub type Scumuxio037R = crate::FieldReader;
#[doc = "Field `SCUMUXIO037` writer - SCU_MUX_IO037"]
pub type Scumuxio037W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMUXIO038` reader - SCU_MUX_IO038"]
pub type Scumuxio038R = crate::FieldReader;
#[doc = "Field `SCUMUXIO038` writer - SCU_MUX_IO038"]
pub type Scumuxio038W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUMUXIO039` reader - SCU_MUX_IO039"]
pub type Scumuxio039R = crate::FieldReader;
#[doc = "Field `SCUMUXIO039` writer - SCU_MUX_IO039"]
pub type Scumuxio039W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SCU_MUX_IO032"]
    #[inline(always)]
    pub fn scumuxio032(&self) -> Scumuxio032R {
        Scumuxio032R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO033"]
    #[inline(always)]
    pub fn scumuxio033(&self) -> Scumuxio033R {
        Scumuxio033R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO034"]
    #[inline(always)]
    pub fn scumuxio034(&self) -> Scumuxio034R {
        Scumuxio034R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO035"]
    #[inline(always)]
    pub fn scumuxio035(&self) -> Scumuxio035R {
        Scumuxio035R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO036"]
    #[inline(always)]
    pub fn scumuxio036(&self) -> Scumuxio036R {
        Scumuxio036R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO037"]
    #[inline(always)]
    pub fn scumuxio037(&self) -> Scumuxio037R {
        Scumuxio037R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO038"]
    #[inline(always)]
    pub fn scumuxio038(&self) -> Scumuxio038R {
        Scumuxio038R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO039"]
    #[inline(always)]
    pub fn scumuxio039(&self) -> Scumuxio039R {
        Scumuxio039R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SCU_MUX_IO032"]
    #[inline(always)]
    pub fn scumuxio032(&mut self) -> Scumuxio032W<Scu410Spec> {
        Scumuxio032W::new(self, 0)
    }
    #[doc = "Bits 4:6 - SCU_MUX_IO033"]
    #[inline(always)]
    pub fn scumuxio033(&mut self) -> Scumuxio033W<Scu410Spec> {
        Scumuxio033W::new(self, 4)
    }
    #[doc = "Bits 8:10 - SCU_MUX_IO034"]
    #[inline(always)]
    pub fn scumuxio034(&mut self) -> Scumuxio034W<Scu410Spec> {
        Scumuxio034W::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_MUX_IO035"]
    #[inline(always)]
    pub fn scumuxio035(&mut self) -> Scumuxio035W<Scu410Spec> {
        Scumuxio035W::new(self, 12)
    }
    #[doc = "Bits 16:18 - SCU_MUX_IO036"]
    #[inline(always)]
    pub fn scumuxio036(&mut self) -> Scumuxio036W<Scu410Spec> {
        Scumuxio036W::new(self, 16)
    }
    #[doc = "Bits 20:22 - SCU_MUX_IO037"]
    #[inline(always)]
    pub fn scumuxio037(&mut self) -> Scumuxio037W<Scu410Spec> {
        Scumuxio037W::new(self, 20)
    }
    #[doc = "Bits 24:26 - SCU_MUX_IO038"]
    #[inline(always)]
    pub fn scumuxio038(&mut self) -> Scumuxio038W<Scu410Spec> {
        Scumuxio038W::new(self, 24)
    }
    #[doc = "Bits 28:30 - SCU_MUX_IO039"]
    #[inline(always)]
    pub fn scumuxio039(&mut self) -> Scumuxio039W<Scu410Spec> {
        Scumuxio039W::new(self, 28)
    }
}
#[doc = "Multi-Function Pin Control \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`scu410::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu410::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu410Spec;
impl crate::RegisterSpec for Scu410Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu410::R`](R) reader structure"]
impl crate::Readable for Scu410Spec {}
#[doc = "`write(|w| ..)` method takes [`scu410::W`](W) writer structure"]
impl crate::Writable for Scu410Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU410 to value 0"]
impl crate::Resettable for Scu410Spec {}

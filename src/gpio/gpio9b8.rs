#[doc = "Register `GPIO9B8` reader"]
pub type R = crate::R<Gpio9b8Spec>;
#[doc = "Register `GPIO9B8` writer"]
pub type W = crate::W<Gpio9b8Spec>;
#[doc = "Field `GPIO168ReadPrivilegeOfMaster` reader - GPIO168 Read Privilege of Master"]
pub type Gpio168readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO168ReadPrivilegeOfMaster` writer - GPIO168 Read Privilege of Master"]
pub type Gpio168readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO169ReadPrivilegeOfMaster` reader - GPIO169 Read Privilege of Master"]
pub type Gpio169readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO169ReadPrivilegeOfMaster` writer - GPIO169 Read Privilege of Master"]
pub type Gpio169readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO170ReadPrivilegeOfMaster` reader - GPIO170 Read Privilege of Master"]
pub type Gpio170readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO170ReadPrivilegeOfMaster` writer - GPIO170 Read Privilege of Master"]
pub type Gpio170readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO171ReadPrivilegeOfMaster` reader - GPIO171 Read Privilege of Master"]
pub type Gpio171readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO171ReadPrivilegeOfMaster` writer - GPIO171 Read Privilege of Master"]
pub type Gpio171readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO168 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio168read_privilege_of_master(&self) -> Gpio168readPrivilegeOfMasterR {
        Gpio168readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO169 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio169read_privilege_of_master(&self) -> Gpio169readPrivilegeOfMasterR {
        Gpio169readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO170 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio170read_privilege_of_master(&self) -> Gpio170readPrivilegeOfMasterR {
        Gpio170readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO171 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio171read_privilege_of_master(&self) -> Gpio171readPrivilegeOfMasterR {
        Gpio171readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO168 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio168read_privilege_of_master(
        &mut self,
    ) -> Gpio168readPrivilegeOfMasterW<Gpio9b8Spec> {
        Gpio168readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO169 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio169read_privilege_of_master(
        &mut self,
    ) -> Gpio169readPrivilegeOfMasterW<Gpio9b8Spec> {
        Gpio169readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO170 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio170read_privilege_of_master(
        &mut self,
    ) -> Gpio170readPrivilegeOfMasterW<Gpio9b8Spec> {
        Gpio170readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO171 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio171read_privilege_of_master(
        &mut self,
    ) -> Gpio171readPrivilegeOfMasterW<Gpio9b8Spec> {
        Gpio171readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#42\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio9b8Spec;
impl crate::RegisterSpec for Gpio9b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio9b8::R`](R) reader structure"]
impl crate::Readable for Gpio9b8Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio9b8::W`](W) writer structure"]
impl crate::Writable for Gpio9b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO9B8 to value 0xffff_ffff"]
impl crate::Resettable for Gpio9b8Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}

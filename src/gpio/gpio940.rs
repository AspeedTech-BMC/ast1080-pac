#[doc = "Register `GPIO940` reader"]
pub type R = crate::R<Gpio940Spec>;
#[doc = "Register `GPIO940` writer"]
pub type W = crate::W<Gpio940Spec>;
#[doc = "Field `GPIO048ReadPrivilegeOfMaster` reader - GPIO048 Read Privilege of Master"]
pub type Gpio048readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO048ReadPrivilegeOfMaster` writer - GPIO048 Read Privilege of Master"]
pub type Gpio048readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO049ReadPrivilegeOfMaster` reader - GPIO049 Read Privilege of Master"]
pub type Gpio049readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO049ReadPrivilegeOfMaster` writer - GPIO049 Read Privilege of Master"]
pub type Gpio049readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO050ReadPrivilegeOfMaster` reader - GPIO050 Read Privilege of Master"]
pub type Gpio050readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO050ReadPrivilegeOfMaster` writer - GPIO050 Read Privilege of Master"]
pub type Gpio050readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO051ReadPrivilegeOfMaster` reader - GPIO051 Read Privilege of Master"]
pub type Gpio051readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO051ReadPrivilegeOfMaster` writer - GPIO051 Read Privilege of Master"]
pub type Gpio051readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO048 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio048read_privilege_of_master(&self) -> Gpio048readPrivilegeOfMasterR {
        Gpio048readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO049 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio049read_privilege_of_master(&self) -> Gpio049readPrivilegeOfMasterR {
        Gpio049readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO050 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio050read_privilege_of_master(&self) -> Gpio050readPrivilegeOfMasterR {
        Gpio050readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO051 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio051read_privilege_of_master(&self) -> Gpio051readPrivilegeOfMasterR {
        Gpio051readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO048 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio048read_privilege_of_master(
        &mut self,
    ) -> Gpio048readPrivilegeOfMasterW<Gpio940Spec> {
        Gpio048readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO049 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio049read_privilege_of_master(
        &mut self,
    ) -> Gpio049readPrivilegeOfMasterW<Gpio940Spec> {
        Gpio049readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO050 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio050read_privilege_of_master(
        &mut self,
    ) -> Gpio050readPrivilegeOfMasterW<Gpio940Spec> {
        Gpio050readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO051 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio051read_privilege_of_master(
        &mut self,
    ) -> Gpio051readPrivilegeOfMasterW<Gpio940Spec> {
        Gpio051readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#12\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio940::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio940::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio940Spec;
impl crate::RegisterSpec for Gpio940Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio940::R`](R) reader structure"]
impl crate::Readable for Gpio940Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio940::W`](W) writer structure"]
impl crate::Writable for Gpio940Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO940 to value 0xffff_ffff"]
impl crate::Resettable for Gpio940Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
